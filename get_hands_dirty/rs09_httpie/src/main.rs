use clap::Parser;
use anyhow::{anyhow, Result};
use colored::*;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// 定义 HTTPie 的 CLI 的主入口，它包含若干子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助信息

/// a native httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 定义子命令，分别对应 HTTP 的 GET 和 POST 请求
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

// get 子命令

/// feed get with an url and we will receive the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[arg(value_parser = parse_url)]
    url: String,
}

// post 子命令

/// feed post with an url and optional key=value pairs. 
/// We will post the data as JSON, and retrieve the response for you.
#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 URL
    #[arg(value_parser = parse_url)]    // 添加自定义解析函数
    url: String,
    /// HTTP 请求的 body
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, Clone, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行 split，这会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

/// 处理 get 命令
async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

/// 处理 post 命令
async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }

    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

/// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

/// 打印服务器返回的 HTTP header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    print!("\n");
}

/// 尝试获取服务器返回值的 Content-Type，如果获取失败，则返回 None
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers().get(header::CONTENT_TYPE).map(|v| v.to_str().unwrap().parse().unwrap())
}

/// 打印HTTP请求返回的 HTTP body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        // 对于 application/json 响应，则将其序列化为 pretty JSON，并打印
        Some(v) if v == mime::APPLICATION_JSON => {
            print_with_syntax(&(jsonxf::pretty_print(body).unwrap()));
        }
        // 对于其他响应，则直接打印文本
        _ => println!("{}", body),
    }
}

/// 打印 Response 内容
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

fn print_with_syntax(text: &String) {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(text) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 通过定义 #[derive(Parser)] 可以自动生成 parse 函数
    let opts = Opts::parse();
    let mut headers = header::HeaderMap::new();

    // 添加默认请求头
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);

    // 生成一个 HTTP 客户端
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}


// 仅在 cargo test 时才编译运行这些测试代码
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(parse_kv_pair("a=1").unwrap(), KvPair { k: "a".into(), v: "1".into() });
        assert_eq!(parse_kv_pair("b=").unwrap(), KvPair { k: "b".into(), v: "".into() });
    }

}
use std::fs;

// cargo run -- https://www.rust-lang.org rust.md
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args:Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <url> <output>", args[0]);
        return Ok(())
    }

    args.iter().for_each(|arg| {
        println!("{}", arg);
    });

    // let url = "https://www.rust-lang.org/";
    // let output = "rust.md";
    let url = &args[1];
    let output = &args[2];

    println!("Fetching {}", url);
    // 访问命名空间或者对象的静态函数使用`::`
    // 访问结构体的成员函数或者变量使用`.`
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}

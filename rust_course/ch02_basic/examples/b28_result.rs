use std::{fs::File, io::{self, ErrorKind, Read}};

fn main() {
    sample();
    unwrap_expect();
    throws_error();
    return_none();
}

fn sample() {
    let f = File::open("sample.txt");

    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("sample.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

// 失败就 panic: unwrap 和 expect
fn unwrap_expect() {
    // 如果返回成功的话，返回 Ok 中的值，否则 panic
    // let f = File::open("hello.txt").unwrap();

    // 如果返回成功的话，返回 Ok 中的值，否则 panic 并带上指定错误
    let f = File::open("unwrap_expect.txt").expect("Failed to open unwrap_expect.txt");
    dbg!(f);
}

// 传播错误
fn throws_error() {
    let r = read_username_from_file().unwrap();
    dbg!(r);

    let r = read_username_from_file_simplify().unwrap();
    dbg!(r);

    let r = read_username_from_file_chain().unwrap();
    dbg!(r);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("read_username_from_file.txt");

    let mut f = match f {
        // 打开文件成功，将 file 句柄复制给 f
        Ok(file) => file,
        // 打开文件失败，将错误返回（向上传播）
        Err(error) => return Err(error),
    };
    
    // 创建动态字符串
    let mut s = String::new();
    // 从 f 文件句柄中读取数据到 s 中
    match f.read_to_string(&mut s) {
        // 读取成功，返回 Ok(s)
        Ok(_) => Ok(s),
        // 读取失败，将错误返回（向上传播）
        Err(e) => Err(e),
    }
}

// 简化代码
fn read_username_from_file_simplify() -> Result<String, io::Error> {
    let mut f = File::open("read_username_from_file_simplify.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 链式调用
fn read_username_from_file_chain() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("read_username_from_file_chain.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// ? 用于 Option 的返回
fn return_none() {
    let arr = [1, 2, 3];
    let v = first(&arr);
    dbg!(v);
}

fn first(arr: &[i32]) -> Option<&i32> {
    let v = arr.get(0)?;
    Some(v)
}

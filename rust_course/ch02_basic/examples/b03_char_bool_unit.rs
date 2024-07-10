fn main() {
    char();
    boolean();
    unit();
}

// 字符
// 在 Rust 中，字符不仅仅是 ASCII ，所有 Unicode 字符都是有效的字符。
fn char() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!("{c} {z} {g} {heart_eyed_cat}");
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
}

// 布尔
fn boolean() {
    let t = true;

    if t {
        println!("t is true");
    } else {
        println!("t is false");
    }
}

// 单元类型 ()
// main 函数的返回值是 ()
// println!() 的返回值也是 ()
fn unit() {
    let u = ();
    println!("u is {:?}", u);
}
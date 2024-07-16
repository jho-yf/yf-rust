// 获取变量的引用，称之为借用 borrowing
fn main() {
    ref_and_deref();
    immutable_ref();
    mutable_ref();
    one_mutable_ref();
    immutable_ref_or_mutable_ref();
    dangling_ref();
}

// 引用 和 解引用
// 常规引用是一个指针类型，指向对象存储的内存地址
fn ref_and_deref() {
    let x = 5;

    // 引用，y 为指针类型
    let y = &x;

    assert_eq!(5, x);
    // 解引用，解出引用所指向的值
    assert_eq!(5, *y);
}

// 不可变引用
fn immutable_ref() {
    let s1 = String::from("value");
    // 将 s1 的引用传递给 calculate_length 函数，而不是把 s1 移动给 calculate_length 函数
    // & 符号表示引用，它允许你使用值，但不获取所有权
    let len = calculate_length(&s1);
    assert_eq!(5, len);
}

fn calculate_length(s: &String) -> usize {  // s 是一个对 String 的引用
    s.len()
}   // 这里 s 离开作用域，但是它并不拥有引用值的所有权，所以此处指向的值不会被丢弃

// 可变引用
fn mutable_ref() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

// 可变引用同时只能存在一个
fn one_mutable_ref() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;             编译会报错

    println!("{}", r1);

    // 利用作用域解决
    {
        let r2 = &mut s;
        println!("{}", r2);
    }
}

// 可变引用与不可变引用不能同时存在
fn immutable_ref_or_mutable_ref() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &s; 编译会报错

    println!("{}", r1);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 在最新编译器中，r1 和 r2 作用域都结束，所以 r3 可以被创建
    // 在最新编译器中，Rust 编译器能够知道 r1 和 r2 不再被使用

    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
}

fn change(s : &mut String) {
    s.push_str(", world");
}

// 悬垂引用 / 悬空指针
// 指针指向某个值后，这个值被释放掉，但指针仍然存在，其指向的内存可能不存在任何值或已被其他变量重新使用
fn dangling_ref() {
    let ref_to_nothing = no_dangle();
    println!("{}", ref_to_nothing);
}

// 以下函数会出现编译异常，&s 的生命周期比返回值 s 的生命周期短，会成为垂悬引用
// fn dangle() -> &String {
//     let s: String = String::from("hello");

//     &s   // 这里返回字符串 s 的引用
// }        // 这里 s 离开作用域并被丢弃，其内存被释放。

fn no_dangle() -> String {
    let s: String = String::from("hello");

    s
}
fn main() {
    // 所有权原则
    //  1. Rust 中每个值都被一个变量所拥有，该变量被称为所有者。
    //  2. 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
    //  3. 当所有者离开作用域时，Rust 会自动释放内存

    scope();

    string();
}

// 作用域
fn scope() {
    let s = "hello";
    println!("{}", s);

    {
        let s = "world";
        println!("{}", s);
    }   // 该作用域结束，s不再有效

    println!("{}", s);  
}   // 该作用域结束，s不再有效

// String 类型
fn string() {
    // s 为 &str 类型，字符串字面值
    // 字符串字面值是不可变的，因为会被硬编码到程序代码中
    // 并非所有字符串的值都能在编码时得知
    let s = "string...";
    println!("s: {s}");

    // s1 为 String 类型，动态字符串类型
    // :: 是一种调用操作符，这里表示调用 String 模块中的 form 方法
    // String 类型的值存储在堆上，因此它是动态的
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    println!("s1: {s1}");
}
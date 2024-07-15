fn main() {
    // 所有权原则
    //  1. Rust 中每个值都被一个变量所拥有，该变量被称为所有者。
    //  2. 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
    //  3. 当所有者离开作用域时，Rust 会自动释放内存

    println!("=============== 作用域 ===============");
    scope();

    println!("=============== String 类型 ===============");
    string();

    println!("=============== 自动拷贝 ===============");
    auto_copy();

    println!("=============== 转移所有权 ===============");
    trans_ownership();

    println!("=============== 深拷贝 ===============");
    deep_copy();

    println!("=============== 浅拷贝 ===============");
    shallow_copy();

    println!("=============== 函数传值与返回 ===============");
    value_passing_returning();
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

// 自动拷贝
fn auto_copy() {
    // 这里不会发生所有权转移
    // 因为整数是大小固定的简单值，都存储在栈上，因此两个值可以通过自动拷贝的方式赋值

    let x = 5;
    let y = x;
    println!("x pointer: {:p}, y pointer: {:p}", &x, &y);
    println!("x value pointer: {:p}, y pointer: {:p}", &x, &y);
}

// 移动所有权
fn trans_ownership() {
    // 当 s1 被赋予 s2 后，Rust 认为 s1 已经不再有效，也就是把值内存的所有权从 s1 转移到 s2
    // 这样子离开当前作用域的时候，Rust 便知道只需要释放 s2 的内存，而无需释放 s1 的内存
    // 防止 二次释放 导致内存污染
    let s1 = String::from("hello");
    println!("\"hello\" pointer: {:p}, s1 pointer: {:p}", s1.as_ptr(), &s1);
    
    // s1 的所有权被转移给 s2，s1 便不再可以使用
    let s2 = s1;
    println!("\"hello\" pointer: {:p}, s2 pointer: {:p}", s2.as_ptr(), &s2);

    // x 只是引用了存储在二进制中的字符串，并没有持有所有权
    let x: &str = "hello";
    // 仅仅对 x 的引用进行了拷贝，此时 y 和 x都引用了同一个字符串
    let y = x;
    println!("x = {x}, y = {y}");
    println!("x pointer: {:p}, y pointer: {:p}", x.as_ptr(), y.as_ptr());
}

// 深拷贝：复制堆上的数据
fn deep_copy() {
    let s1 = String::from("value");
    let s2 = s1.clone();
    println!("s1: {s1}, s2: {s2}");
    println!("s1 pointer: {:p}, s2 pointer: {:p}", s1.as_ptr(), s2.as_ptr())
}

// 浅拷贝：复制栈上的数据，Rust 自动帮我们调用 clone 方法
// Rust 有个 Copy trait，如果类型实现了 Copy trait，那么 Rust 会自动帮我们调用 clone 方法
// 即如果一个变量拥有 Copy trait，一个旧的变量在被赋值给其他变量后仍然可用
/*
    实现 Copy trait 的类型：
        - 所有整数类型
        - 布尔类型
        - 所有浮点数类型
        - 字符类型，char
        - 所有元组类型，其中元组中的每个元素都实现了 Copy trait
        - 不可变引用 &T
*/
fn shallow_copy() {
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
    println!("x pointer: {:p}, y pointer: {:p}", &x, &y);
}

// 函数传值与返回
fn value_passing_returning() {
    let s = String::from("hello");       // s 进入作用域
    takes_ownership(s);                // s 的值移动到函数里，此时 s 失去了所有权

    let x = 5;                             // x 进入作用域
    makes_copy(x);
    println!("x = {x}");

    let s1 = gives_ownership();
    let s2 = String::from("value");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {s1}, s3 = {s3}");
}

fn takes_ownership(some_str: String) {  // some_str 进入作用域
    println!("{}", some_str);
}                                       // 这里 some_str 移出作用域并调用 drop 方法，占有的内存被释放

fn makes_copy(some_int: i32) {          // some_int 进入作用域
    println!("{}", some_int);
}                                       // 这里 some_int 移出作用域，不会有其他特殊操作

// 将返回值的所有权移动给调用它的函数
fn gives_ownership() -> String {
    let some_str = String::from("value");    // some_str 进入作用域
    some_str                    // 返回 some_str 并移出给调用的函数
}

fn takes_and_gives_back(some_str: String) -> String {   // some_str 进入作用域
    some_str                    // 返回 some_str 并移出给调用的函数
}
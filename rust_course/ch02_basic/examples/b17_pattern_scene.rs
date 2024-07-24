use std::vec;

/*
    模式是 Rust 中的特殊语法，它用来匹配类型中的结构和数据，它往往和 match 语句一起使用。
    模式一般由以下内容组成：
        1. 字面值
        2. 解构的数组、枚举、结构体或者元组
        3. 变量
        4. 通配符
        5. 占位符
*/
fn main() {
    // 所有可能用到模式的地方
    match_branch();
    if_let_branch();
    while_let();
    for_loop();
    let_();
    func_param();
}

// match 分支
// match 的每一个分支就是一个 模式
fn match_branch() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

// if let 分支
// if let 往往用于匹配一个模式而忽略其他模式
fn if_let_branch() {
    let v = Some(100);
    if let Some(100) = v {
        println!("v is 100");
    }
}

// while let 条件循环
fn while_let() {
    // 动态数组
    let mut stack = Vec::new();

    // 插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        dbg!(top);
    }
}

// for 循环
// 使用 (index, value) 形式的元组匹配
fn for_loop() {
    let v = vec![1, 2, 3];
    for (idx, value) in v.iter().enumerate() {
        println!("index: {}, value: {}", idx, value);
    }
}

// let 语句
fn let_() {
    let (x, y, z) = (1, 2, 3);
    dbg!(x, y, z);
}

// 函数参数
// 函数参数也是模式
fn func_param() {
    let point = (3, 5);
    print(&point);

    fn print(&(x, y): &(i32, i32)) {
        dbg!(x, y);
    }
}
fn main() {
    println!("{}", add_with_extra(1, 2));

    stmt();

    expr();

    assert_eq!(return_unit_type(), ());
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    // 语句
    let x = x + 1;
    // 语句
    let y = y + 2;
    // 表达式
    x + y
}

// 语句
fn stmt() {
    let a = 8;
    println!("{a}");
    let b: Vec<f64> = Vec::new();
    let (a, c) = ("hi", false);
    println!("{a}, {b:?}, {c}");
}

// 表达式
fn expr() {
    let y = {
        let x = 3;
        // 表达式
        x + 1
    };

    println!("The value of y is: {y}")
}

// 如果表达式不返回任何值，会隐式返回一个 ()
fn return_unit_type() {
    let x = 1;
    // if 语句快也是一个表达式，因此也可以用于赋值，类似三元运算符
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 简写
    let z = if x % 2 == 1 { "odd" } else { "even" };
    println!("y = {y}, z = {z}");
}
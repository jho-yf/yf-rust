use std::fmt::Debug;

// 函数名 和 变量名 使用 蛇形命名法
// 函数位置随便放
fn main() {
    another_fn(1, 2.0);
    println!("1 plus five is {}", plus_five(1));

    println!("{}", plus_or_minus(10));

    report("Hello Rust");
    report(100_i32);

    let mut text = String::from("Hello");
    clear(&mut text);
    println!("{:?}", text);

    forever();
}

// 函数参数
// 函数每个参数都需要标注类型
fn another_fn(x: i32, y: f32) {
    println!("x: {}, y: {}", x, y);
}

// 函数返回值：函数名后面加 -> 返回值类型
fn plus_five(x: i32) -> i32 {
    x + 5
}

// 提前返回要加 return
fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        // 提前返回
        return x - 5
    }

    x + 5
}

// 无返回值：隐式返回 ()
fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

// 无返回值：显示返回 ()
fn clear(text: &mut String) -> () {
    *text = String::from("");
}

// 永不返回的发散函数 !
fn forever() -> ! {
    loop {
        println!("forever");
    }
}
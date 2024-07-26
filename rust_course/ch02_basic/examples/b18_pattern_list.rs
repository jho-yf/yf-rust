fn main() {
    // 此示例会将所有不同类型的模式例子列举出来，方便查阅
    match_literal();
    match_variable();
    match_multiple_patterns();
    match_range();
    destructure_struct();
    destructure_enum();
    destructure_nested();
    destructure_struct_and_tuple();
    destructure_array();
    ignore_pattern();
    match_guard();
    at_bind();
}

// 匹配字面值
fn match_literal() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// 匹配命名变量
fn match_variable() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // 由于变量遮蔽，这里的 y 和上面的 y 不是同一个变量
        // 只是将 Some(5) 中的 5 匹配到新变量 y 上面
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y= {:?}", x, y);
}

// 单分支多模式
fn match_multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let y = Some(2);
    match y {
        Some(1) | Some(2) => println!("one or two"),
        Some(3) => println!("three"),
        _ => println!("anything"),
    }
}

// 通过序列 ..= 匹配值的范围
fn match_range() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let y = Some(50);
    match y {
        Some(1..=5) => println!("one through five"),
        _ => println!("something else"),
    }

    let c = 'c';
    match c {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// 解构结构体
fn destructure_struct() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // 固定某个字段的匹配方式
    match p {
        // 匹配 x 为 任意值，y 为 0
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // 匹配 x 为 0，y 为 任意值
        Point { x: 0, y } => println!("On the y axis at {}", y),
        // 匹配 x 和 y 都不为 0
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// 解构枚举
fn destructure_enum() {
    let msg = Message::Quit;
    match_enum(msg);
    let msg = Message::Move { x: 0, y: 0 };
    match_enum(msg);
    let msg = Message::Write(String::from("hello"));
    match_enum(msg);
    let msg = Message::ChangeColor(0, 160, 255);
    match_enum(msg);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn match_enum(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            )
        },
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r, g, b
            )
        },
    }
}

// 结构嵌套的结构体和枚举
fn destructure_nested() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    match Message::ChangeColor(Color::Hsv(0, 160, 255)) {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        },
        _ => (),
    }

    match Message::ChangeColor(Color::Rgb(0, 160, 255)) {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        },
        _ => (),
    }

    match Message::Quit {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        },
        _ => (),
    }

    match (Message::Move{ x: 0, y: 0 }) {
        Message::Move{ x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        },
        _ => (),
    }

    match Message::Write("hello".to_string()) {
        Message::Write(str) => println!("Text message: {}", str),
        _ => (),
    }    
}

// 解构结构体和元组
fn destructure_struct_and_tuple() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point{ x, y }) = ((3, 10), Point{ x: 3, y: -10 });
    dbg!(feet, inches, x, y);
}

// 解构数组
fn destructure_array() {
    // 定长数组
    let arr: [u16; 2] = [1, 2];
    let [a, b] = arr;
    assert_eq!(a, 1);
    assert_eq!(b, 2);

    // 不定长数组
    let arr: &[u16] = &[1, 2];
    if let [x, ..] = arr {
        assert_eq!(x, &1);
    }
    if let [.., y] = arr {
        assert_eq!(y, &2);
    }

    let arr: &[u16] = &[];
    assert!(matches!(arr, [..]));
    if let [_, ..] = arr {
        panic!("unreachable!");
    }
    assert!(!matches!(arr, [_, ..]));
}

// 忽略模式中的值
fn ignore_pattern() {
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        },
    }

    println!("setting is {:?}", setting_value);

    // 忽略元组中的值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        },
    }

    // 忽略未使用的变量
    let _x = 100;

    // 使用 _ 可以防止 s 值的所有权转移
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }

    // 使用 .. 忽略剩余的值
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point { x: 0, y: 1, z: 2 };
    dbg!(origin.x, origin.y, origin.z);
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // 使用 .. 忽略中间的值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}

// 匹配守卫
// 匹配守卫是一个位于 match 分支模式之后的额外 if 条件，它能为分支模式提供进一步的匹配条件
fn match_guard() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

// @ 绑定
fn at_bind() {
    enum Message {
        Hello { id: i32 }
    }

    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => println!("Found some other id: {}", id),
    } 

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    // 绑定变量 `p` , 同时解构 Point
    let p @ Point { x: px, y: py } = Point { x: 0, y: 0 };
    println!("Point p is at ({}, {})", px, py);
    println!("Point p is at {:?}", p);

    let p = Point { x: 10, y: 5 };
    if let p @ Point {x: 10, y} = p {
        println!("x is 10, y: {} in {:?}", y, p);
    } else {
        println!("x is not 10");
    }

    match 1 {
        num @ (1 | 2) => {
            println!("num is {}", num);
        },
        _ => {}
    }
}
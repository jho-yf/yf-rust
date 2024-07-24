fn main() {
    sample();
    match_bind();
    if_let();
    matches_macro();
    variable_shadowing();
}

// 示例
fn sample() {
    enum Direction {
        East,
        West,
        North,
        South,
    }

    let _ = Direction::East;
    let _ = Direction::West;
    let _ = Direction::North;
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::West | Direction::North => {
            println!("West or North")
        },
        _ => println!("South"),
    }
}

// 模式绑定
fn match_bind() {
    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColor(u16, u16, u16),
    }

    let actions = [
        Action::Say("Hello".to_string()),
        Action::MoveTo(10, 20),
        Action::ChangeColor(255, 0, 0),
    ];

    for action in actions.iter() {
        match action {
            Action::Say(s) => println!("Say: {}", s),
            Action::MoveTo(x, y) => println!("Move to ({}, {})", x, y),
            Action::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
        }
    }
}

// if let 匹配
fn if_let() {
    let value = Some(3u8);
    
    // 匹配多个条件
    match value {
        Some(3) => println!("three"),
        _ => (),
    }
    // 只匹配一个条件，其他忽略
    if let Some(3) = value {
        println!("three");
    }
    println!("value: {:?}", value);
}

// matches! 宏
fn matches_macro() {
    #[derive(Debug)]
    enum MyEnum {
        Foo,
        Bar
    }
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let _ = v.iter().filter(|x| matches!(x, MyEnum::Foo));

    let foo = 'f';
    assert!(matches!(foo, 'f'));
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 3));
}

// 变量遮蔽
fn variable_shadowing() {
    let age = Some(30);
    println!("匹配前 age: {:?}", age);
    if let Some(age) = age {
        println!("匹配出来 age: {:?}", age);
    }
    println!("匹配后 age: {:?}", age);
}
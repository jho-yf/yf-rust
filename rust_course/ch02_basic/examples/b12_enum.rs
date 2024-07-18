fn main() {
    enum_example();
    option();
    match_option();
}

#[derive(Debug)]
enum PokerSuit {
    Clubs(u8),
    Diamonds(u8),
    Hearts(char),
    Spades(char),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_example() {
    let clubs = PokerSuit::Clubs(3);
    let diamonds = PokerSuit::Diamonds(8);
    let heart = PokerSuit::Hearts('A');
    let spades = PokerSuit::Spades('K');

    print_card(clubs);
    print_card(diamonds);
    print_card(heart);
    print_card(spades);

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::Write(String::from("Hello"));
    let m4 = Message::ChangeColor(255, 255, 0);
    print_message(m1);
    print_message(m2);
    print_message(m3);
    print_message(m4);
}

fn print_card(card: PokerSuit) {
    match card {
        PokerSuit::Clubs(x) => println!("Clubs: {}", x),
        PokerSuit::Diamonds(x) => println!("Diamonds: {}", x),
        PokerSuit::Hearts(x) => println!("Hearts: {}", x),
        PokerSuit::Spades(x) => println!("Spades: {}", x),
    }
}

fn print_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(s) => println!("Write: {}", s),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
    }
}

// Option 枚举
fn option() {
    let some_num = Some(5);
    let some_str = Some("a string");
    // 使用 None 需要告诉编译器，Option<T> 是什么类型
    let absent_num: Option<i32> = None;
    dbg!(some_num, some_str, absent_num);
}

// 模式匹配 Option
fn match_option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(five, six, none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}



fn main() {
    match_option();
}

// 匹配 Option<T>
fn match_option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(six, none);
}

fn plus_one(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i + 1),
    }
}
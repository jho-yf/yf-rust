fn main() {
    sample();
}

// 闭包
fn sample() {
    let x = 1;
    let sum = |y| x + y;
    assert_eq!(3, sum(2));
}
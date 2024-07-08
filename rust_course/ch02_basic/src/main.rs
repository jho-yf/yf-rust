fn main() {
    let a = 10;
    let b: i32 = 20;

    // 可以在值后面带上类型,30i32表示数值是 30,类型是 i32
    // mut 是 mutable 的所写,表示变量 c 是可变的
    let mut c = 30i32;
    // 在数值和类型中间添加一个下划线,30_i32表示数值是 30,类型是 i32
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("(a + b) + (c + d) = {}", e)
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

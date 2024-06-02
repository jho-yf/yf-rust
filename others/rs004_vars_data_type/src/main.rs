fn main() {
    // 不可变性
    let _number_a = 1;
    let _number_b: i64 = 54;
    let _number_b: i64 = 53;

    println!("number_b: {}", _number_b);

    // 声明可变
    let mut _number_c = 10;
    _number_c = 5;

    // Shadowing
    let var = 10;
    {
        // 命名空间
        let var = 100;
        println!("inner number_d: {}", var);
    }
    println!("outer number_d: {}", var);

    let var = "hello";
    println!("new var: {}", var)

}

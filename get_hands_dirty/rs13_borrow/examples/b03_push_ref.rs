// 把栈上的变量的引用存在堆上
fn main() {
    // 因为 data 和 v 的值的生命周期是相同的
    // 即使 data 是存放在堆上的，v 是存放在栈上的，也可以将 v 的值引用存在堆上
    let mut data: Vec<&u32> = Vec::new();
    let v = 42;
    data.push(&v);
    println!("{:?}", data);
}
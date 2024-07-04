// 多个可变引用共存
fn main() {
    let mut data = vec![1, 2, 3];

    // 以下反例代码会报错：同一个作用于下有多个可变引用，是不安全的
    // cannot borrow `data` as mutable more than once at a time second mutable borrow occurs here
    for item in data.iter_mut() {
        data.push(*item + 1);
    }
}
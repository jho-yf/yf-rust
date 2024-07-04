// 只读引用 和 可变引用 共存

fn main() {
    // 以下反例代码会报错：cannot borrow `data` as mutable because it is also borrowed as immutable
    // 

    let mut data = vec![1, 2, 3];
    let data1 = vec![&data[0]];
    println!("data[0] = {:p}", &data[0]);

    for i in 0..100 {
        data.push(i);
    }

    println!("data[0] = {:p}", &data[0]);
    println!("boxed: {:p}", &data1)
}
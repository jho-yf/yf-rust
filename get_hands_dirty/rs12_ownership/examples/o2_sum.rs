fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let data1 = data;

    println!("sum of data1: {}", sum(data1));

    // error: borrow of moved value: `data1`
    // println!("data1: {:?}", data1);
    // error: borrow of moved value: `data`
    // println!("sum of data: {}", sum(data));
}

fn sum(data: Vec<u32>) -> u32 {
    // fold 用于将一系列值累积单一结果
    // init: 初始值
    // acc: 累加器，初始值为 0
    // x: 当前值
    data.iter().fold(0, |acc, x| acc + x)
}
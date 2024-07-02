fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let data1 = &data;

    // 值的地址是相同的
    println!("data值的地址：{:p}", &data);
    println!("data1值的地址：{:p}", data1);

    // data 和 data1 引用的地址是不相同
    println!("data变量引用的地址：{:p}", &&data);
    println!("data1变量引用的地址：{:p}", &data1);

    println!("sum = {}", sum(data1));

    println!( "堆上数据的地址: [{:p}, {:p}, {:p}, {:p}]", &data[0], &data[1], &data[2], &data[3] );
}

fn sum(data: &Vec<u32>) -> u32 {
    println!("sum函数中，data值的地址：{:p}", data);
    println!("sum函数中，data变量引用的地址：{:p}", &data);
    data.iter().fold(0, |acc, x| acc + x)
}
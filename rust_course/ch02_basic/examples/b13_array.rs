// array    长度固定，性能好，存储在栈上
// Vector   长度可变，有性能损耗，存储在堆上
fn main() {
    array();
    array_visit();
    array_slice();
    array_demo();
}

// 固定数组
fn array() {
    // 隐式声明类型
    let a = [1, 2, 3, 4, 5];
    // 显式声明类型
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // 初始化某个值重复出现 N 次
    let repeat = [3; 5];

    // 下面这行代码编译会报错，因为 let array = [3;5] 原理是不断 Copy
    // 而 String 类型没有深拷贝
    // let array = [String::from("value");8];

    let strs: [String; 3] = std::array::from_fn(|_i| String::from("value"));

    dbg!(a, months, repeat, strs);
}

// 访问数组元素
fn array_visit() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    dbg!(first, second);
}

// 数组切片
fn array_slice() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn array_demo() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    for item in &arrays {
        print!("{:?}", item);

        for n in item.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..item.len() {
            sum += item[i];
        }
        println!("\t({:?} = {})", item, sum)
    }
}
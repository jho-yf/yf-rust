fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let v = 4;
    if let Some(pos) = find_pos(data, v) {
        println!("Found {} at {}", v, pos)
    }

    // 下面这一行代码会报错：borrow of moved value: `data` 
    // 根据 rust 的所有权规则，main 函数中的data变量将所有权移动到 find_pos() 函数后，就失效了。
    // 编译器会保证 main 函数随后的代码都无法访问 data 变量。
    // println!("data = {:?}", data);

    println!("v = {}", v)
}

fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }

    None
}
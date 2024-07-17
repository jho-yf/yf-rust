// 元组是由多种类型组合到一起形成的
// 元组长度固定
// 元组中元素的顺序是固定的
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    dbg!(tup);

    deconstructing_tuple();
    index_access();
    tuple_return();
}

// 解构元组
fn deconstructing_tuple() {
    let tup = (500, 6.4, 1);
    let (x, .., y) = tup;
    dbg!(x, y);
}

// 索引访问
fn index_access() {
    let tup = (500, 6.4, 1);
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    dbg!(x, y, z);
}

fn tuple_return() {
    let s1 = String::from("hello");
    let (s2, len) = calc_len(s1);
    dbg!(s2, len);
}

// 使用元组返回多个值
fn calc_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
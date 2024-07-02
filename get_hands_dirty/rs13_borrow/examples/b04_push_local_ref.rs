fn main() {
    let mut data: Vec<&u32> = Vec::new();
    push_local_ref(&mut data);
    println!("data: {:?}", data);
}

// 以下代码会报错，因为不能引用生命周期更短的变量
// v` does not live long enough 
// borrowed value does not live long enough
fn push_local_ref(data: &mut Vec<&u32>) {
    let v = 42;
    data.push(&v);
}
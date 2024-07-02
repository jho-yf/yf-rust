fn main() {
    let r = local_ref();
    println!("r: {:p}", r);
}

// 以下代码会报错，因为不能引用生命周期更短的变量
// cannot return reference to local variable `a`
// returns a reference to data owned by the current function
fn local_ref<'a>() -> &'a i32 {
    let a = 42;
    &a
}
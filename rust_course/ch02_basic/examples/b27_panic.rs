fn main() {
    panic_sample();
}

// 主动调用
fn panic_sample() {
    // panic! 宏：程序会打印一个错误信息，展开报错点往前的函数调用堆栈，最后退出程序
    // backtrace 栈展开： RUST_BACKTRACE=1 cargo run --example b27_panic 
    panic!("crash and burn");
}


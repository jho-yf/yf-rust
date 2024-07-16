#![allow(unused_variables)]     // 告诉编译器，未使用的变量是允许的
type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

// ! 表明该函数是一个发散函数，不会返回任何值
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    // unimplemented!() 告诉编译器该函数还未实现
    unimplemented!()
}

fn main() {
    let mut f = File::from("foo.txt");
    open(&mut f);
    // read(&mut f, &mut vec![]);
    close(&mut f);
}
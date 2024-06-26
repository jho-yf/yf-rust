/* 
    - 函数的参数类型和返回值的类型都必须显示定义,如果没有返回值可以省略,返回unit
    - 函数最后一个表达式就是返回值,如果在函数内提前返回值,使用return关键字 
    - 如果最后一个表达式添加了`;`,隐含其返回值为unit
*/ 

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

// 函数内部
fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    3.1415926;
}

fn main() {
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));

    let is_pi = pi();
    let is_unit1 = not_pi();
    let is_unit2 = {
        pi();
    };

    println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2)
}

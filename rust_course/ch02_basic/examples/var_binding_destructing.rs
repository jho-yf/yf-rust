fn main() {
    // 变量绑定
    // 变量 a 不可变,一旦绑定值,就不能修改
    let a = "hello world";
    println!("{a}");

    // 变量可变性
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // 使用下划线开头忽略未使用的变量
    let _var1 = 5;
    let var2 = 10;
    println!("var2 is {var2}");

    // 变量解构
    let (a, mut b): (bool, i32) = (true, 1);
    println!("a is {a}, b is {b}");
    b = 2;
    assert_eq!(b, 2);

    // 解构式赋值
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    // _ 代表匹配一个值,因为无需关心具体值是什么
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct{ e, .. } = Struct{ e: 1 };
    assert_eq!([1, 2, 1, 4, 1], [a, b, c, d, e]);

    // 常量与变量的差异
    // 1. 常量不允许使用 mut
    // 2. 常量使用 const 关键字声明，且值的类型必须标注
    // 3. 常量可以在任何作用域生命
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS is {MAX_POINTS}");

    // 变量遮蔽 shadowing
    // Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的
    let shadowing_var = 5;
    // 在 main 函数的作用域内对之前的 shadowing_var 进行遮蔽
    let shadowing_var = shadowing_var + 1;
    
    {
        println!("before shadowing: {shadowing_var}");

        // 在当前的花括号作用域内，对 main 函数的 shadowing_var 进行遮蔽
        let shadowing_var = shadowing_var * 2;

        println!("before shadowing: {shadowing_var}");
    }

    assert_eq!(shadowing_var, 6);

    // 变量遮蔽，变量名复用
    let words = "Rust";
    let words = words.len();
    println!("words is {words}");
}

struct Struct {
    e: i32
}
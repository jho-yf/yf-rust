fn main() {
    int_overflow();
    float();
    nan();
    num_op();
    format_print();
    bit_op();    
    range();
    complex();
}

// 整型溢出
fn int_overflow() {
    // wrapping_* 按照补码循环溢出规则处理
    // 即 u8 最大值是 255，加 20 后变成 19
    let a: u8 = 255;
    let a = a.wrapping_add(20);
    assert_eq!(a, 19);

    // checked_* 溢出后返回 None
    assert_eq!(1_u8.checked_add(1), Some(2));
    assert_eq!(u8::MAX.checked_add(1), None);

    // overflowing_* 溢出后返回溢出值和溢出标志
    assert_eq!(1_u8.overflowing_add(1), (2, false));
    assert_eq!(u8::MAX.overflowing_add(1), (0, true));

    // saturating_* 溢出后返回目标类型的最大值
    assert_eq!(100_u8.saturating_add(1), 101);
    assert_eq!(u8::MAX.saturating_add(127), u8::MAX);
}

// 浮点数
fn float() {
    // 默认 f64
    let f = 2.0;
    println!("{f}");
    // f32
    let f: f32 = 3.0;
    assert_eq!(f, 3.0);

    // 浮点数精度问题, 0.1 + 0.2 并不严格等于 0.2,它们可能在小数点后 N 位存在误差
    assert!(0.1 + 0.2 != 0.3);
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("        0.3: {:x}", abc.2.to_bits());
    println!();

    println!("xyz (f64)");
    println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("        0.3: {:x}", xyz.2.to_bits());
    println!();
    
    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 != xyz.2);
}

// NaN
fn nan() {
    // NaN (not a number) 数学上未定义的结果
    // 所有跟 NaN 交互的操作都会返回 NaN
    // NaN 与 NaN 比较结果都是 false
    let x = (-42.0_f32).sqrt();
    assert_ne!(x, x);
    assert!(x.is_nan());
}

fn num_op() {
    // 数字运算
    assert_eq!(5 + 10, 15);
    assert_eq!(99 - 9, 90);
    assert_eq!(2 * 3, 6);
    assert_eq!(100 / 5, 20);
    assert_eq!(15 % 4, 3);
}

// 格式化输出
fn format_print() {
    // 对于较长的数字，使用下划线(_)来分隔数字，以增强可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 数组第一个元素可以自动推导 f32 类型
    let forty_tows = [
        42.0,
        42f32,
        42.0f32,
        42.0_f32
    ];
    // 控制打印的小数位为2位
    println!("{:.2}", forty_tows[0]);
}

// 位运算
fn bit_op() {
    // 二进制为：00000010
    let a:i32 = 2;
    // 二进制为：00000011
    let b:i32 = 3;

    // & 位与
    println!("a & b = {}", a & b);
    // | 位或
    println!("a | b = {}", a | b);
    // ^ 异或
    println!("a ^ b = {}", a ^ b);
    // ! 位非
    println!("!a = {}", !a);
    // << 左移
    println!("a << 1 = {}", a << b);
    // >> 右移
    println!("a >> 1 = {}", a >> b);

    let mut a = a;
    // 除了 ! 之外，都可以加上 = 进行赋值
    a <<= b;
    println!("(a << b) value is {a}");
    a ^= b;
    println!("(a ^ b) value is {a}");
    a |= b;
    println!("(a | b) value is {a}");
}

// 序列
// 序列只支持用于数据或字符类型，因为他们可以是连续
fn range() {
    for i in 1..=10 {
        println!("{i}");
    }
    for i in 'a'..='z' {
        println!("{i}");
    }
}

// num 库
// num 库提供了一些数学运算的扩展，包括复数、分数、定点数等
// 在 Cargo.toml 中的 [dependencies] 下添加一行 num = "0.4.0"
use num::complex::Complex;
fn complex() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}
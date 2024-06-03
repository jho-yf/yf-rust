/*
    斐波那契数列实现：
        斐波那契数列指的是这样一个数列 0, 1, 1, 2, 3, 5, 8, 13
        特别指出：第0项是0，第1项是第一个1。从第三项开始，每一项都等于前两项之和。
*/

fn next_fib(num1: &mut i32, num2: &mut i32) {
    let c = *num1 + *num2;
    *num1 = *num2;
    *num2 = c;
    println!("next val is {}", num2);
}

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        next_fib(&mut a, &mut b);
        i += 1;

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        next_fib(&mut a, &mut b);
        i += 1;
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        next_fib(&mut a, &mut b);
    }
}

fn main() {
    let n = 10;
    println!("======= fib loop =======");
    fib_loop(n);
    println!("======= fib while =======");
    fib_while(n);
    println!("======= fib for =======");
    fib_for(n);

    let arr = [1, 2, 3];
    assert_eq!(arr[..], [1, 2, 3]);
    assert_eq!(arr[0..=1], [1, 2]);
}

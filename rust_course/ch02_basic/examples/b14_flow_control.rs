fn main() {
    if_example();
    for_example();
    for_collection();
    continue_example();
    break_example();
    while_example();
    loop_example();
}

fn if_example() {
    let condition = true;
    // if 语句快是表达式，使用 if 来赋值，需要保证每个分支的返回值类型一致
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    let n = 6;
    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// for 循环
fn for_example() {
    // for 
    for i in 1..=5 {
        println!("{}", i);
    }
    for i in 1..5 {
        assert_ne!(i, 5);
    }
    for (idx, v) in (1..=5).enumerate() {
        println!("index: {}, value: {}", idx, v);
    }

    // for 循环遍历集合
    
    // 转移所有权，后面的代码不能再使用 collection
    // 对于实现了 copy 特性的数组，不会转移所有权
    // for item in collection
    
    // 不可变借用，只读
    // for item in &collection

    // 可变借用，可以修改 item
    // for item in &mut collection

    let a = [10, 20, 30, 40, 50];
    for (idx, el) in a.iter().enumerate() {
        println!("index: {}, element: {}", idx, el);
    }

    // 无需声明变量
    for _ in 0..5 {
        println!("hello");
    }
}

// 两种循环方式优劣对比
/*
    性能：第一种使用索引访问，会因为边界检查导致性能损耗
    安全：第一种使用索引访问是非连续的，存在一定可能性在两次访问之间，collection 的元素被修改，导致脏数据产生
*/
fn for_collection() {
    let collection = [1, 2, 3, 4, 5];

    for i in 0..collection.len() {
        let item = collection[i];
        println!("{}", item);
    }

    for item in collection {
        println!("{}", item);
    }
}

// continue
fn continue_example() {
    for n in 1..=5 {
        if n % 2 == 0 {
            continue;
        }
        println!("{}", n);
    }
}

// break
fn break_example() {
    for n in 1..=5 {
        if n % 2 == 0 {
            break;
        }
        println!("{}", n);
    }
}

// while
fn while_example() {
    let mut n = 1;
    while n <= 5 {
        println!("{}", n);
        n += 1;
    }
    println!("while end")
}

// loop
fn loop_example() {
    let mut n = 1;
    loop {
        println!("{}", n);
        n += 1;
        if n > 5 {
            break;
        }
    }
    println!("loop end")
}
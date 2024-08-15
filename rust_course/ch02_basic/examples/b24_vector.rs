use std::vec;

fn main() {
    create_vec();
    update_vec();
    read_vec();
    borrow_vec();
    iterate_vec();
    store_different_type_in_vec();
    vector_method();
    sort_vec();
}

// 创建动态数组
fn create_vec() {
    let mut  v: Vec<i32> = Vec::new();
    v.push(1);
    dbg!(v);

    // 类型自动推断
    let v = vec![1, 2, 3];
    dbg!(v);

    // 默认值为0，初始化长度为3
    let v = vec![1; 3];
    let v_from = Vec::from([1, 1, 1]);
    assert_eq!(v, v_from);

    let mut v: Vec<i32> = Vec::with_capacity(16);
    // 附加数据到 v
    v.extend([1, 2, 3]);

    let v = [1, 2, 3].to_vec();
    dbg!(v);
}

// 更新 Vector
fn update_vec() {
    let mut v = Vec::new();
    v.push(4);
    dbg!(v);
}

// 从 Vector 中读取元素
fn read_vec() {
    let v = vec![1, 2, 3];
    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

// 同时借用多个数组元素
fn borrow_vec() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    dbg!(first);

    v.push(6);

    // 这时候再使用 first 会出现编译异常
    // 因为数组大小是可变的，当旧数组的大小不够用时，rust 会重新分配一块更大的内存空间，然后把旧数据拷贝过去
    // 此时，first 就会指向一块无效的内存
    // println!("The first element is: {}", first);
}

// 迭代遍历 Vector 中的元素
fn iterate_vec() {
    let v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("{}", i);
    }

    // 在迭代的过程中，修改元素
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }
    dbg!(v);
}

// 存储不同类型的元素
fn store_different_type_in_vec() {
    trait IpAddr {
        fn display(&self);
    }

    struct V4(String);
    impl IpAddr for V4 {
        fn display(&self) {
            println!("ipv4: {:?}", self.0);
        }
    }

    struct V6(String);
    impl IpAddr for V6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0);
        }
    }

    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}

// Vector 常用方法
fn vector_method() {
    let mut v = Vec::with_capacity(16);

    // 附加数据
    v.extend([1, 2, 3]);
    println!("v 的长度是：{}，容量是：{}", v.len(), v.capacity());

    // 调整容量
    v.reserve(100);
    println!("v 的长度是：{}，容量是：{}", v.len(), v.capacity());

    // 释放剩余容量，一般情况下，不会主动释放
    v.shrink_to_fit();
    println!("v 的长度是：{}，容量是：{}", v.len(), v.capacity());

    dbg!(v.is_empty());
    // 在指定索引插入数据
    v.insert(2, 100);
    // 移除指定位置的的元素并返回
    assert_eq!(v.remove(2), 100);
    assert_eq!(v.pop(), Some(3));
    assert_eq!(v.pop(), Some(2));
    v.clear();
    assert_eq!(v.pop(), None);

    let mut v1 = vec![1, 2, 3];
    // 将 v1 的数据移动到 v 中
    v.append(&mut v1);
    assert!(v1.is_empty());

    // 截断到指定长度，多余的元素会被删除
    v.truncate(2);
    assert_eq!(v, vec![1, 2]);

    // 保留满足条件的元素，删除其他的元素
    v.retain(|x| *x % 2 == 0);
    assert_eq!(v, vec![2]);

    // 删除指定范围的元素，同时获取被删除的元素的迭代器
    let mut v = vec![1, 2, 3, 4, 5];
    let mut m: Vec<_> = v.drain(1..=3).collect();
    assert_eq!(v, vec![1, 5]);
    assert_eq!(m, vec![2, 3, 4]);

    // 在指定位置切分
    let m2 = m.split_off(1);
    assert_eq!(m, vec![2]);
    assert_eq!(m2, vec![3, 4]);

    // 数组切片
    let v = vec![1, 2, 3, 4, 5];
    let slice = &v[1..=3];
    assert_eq!(slice, &[2, 3, 4]);
    assert_eq!(v, &[1, 2, 3, 4, 5]);
}

// Vector 的排序
// sort 和 sort_by 都是稳定排序，即相同元素的相对顺序不会改变
// sort_unstable 和 sort_unstable_by 则是不稳定的排序，即相同元素的相对顺序可能会改变
fn sort_vec() {
    // 整数数组排序
    let mut v = vec![1, 5, 2, 4, 3];
    v.sort();
    assert_eq!(v, vec![1, 2, 3, 4, 5]);

    // 浮点数数组排序
    let mut v = vec![1.0, 5.0, 2.0, 4.0, 3.0];
    v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(v, vec![1.0, 2.0, 3.0, 4.0, 5.0]);

    // 结构体数组排序
    #[derive(Debug)]
    struct Person {
        age: u32,
    }

    let mut people = vec![
        Person {
            age: 30,
        },
        Person {
            age: 25,
        },
        Person {
            age: 35,
        },
    ];
    people.sort_by(|a, b| a.age.cmp(&b.age));
    dbg!(people);

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Animal {
        age: u32,
    }
    let mut animals = vec![
        Animal {
            age: 30,
        },
        Animal {
            age: 25,
        },
        Animal {
            age: 35,
        },
    ];
    animals.sort_unstable();
    dbg!(animals);
}
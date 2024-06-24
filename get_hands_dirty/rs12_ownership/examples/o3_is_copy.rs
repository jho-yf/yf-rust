fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    // 所有 iXX uXX usize isize fXX 都实现了 Copy trait
    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();
    is_copy::<f64>();

    // function (实际上是个指针) 实现了 Copy trait
    is_copy::<fn()>();

    // raw pointer (原始指针) 实现了 Copy trait
    // 原始指针指的是两种类型：
    // 1. *const T
    // 2. *mut T
    is_copy::<*const String>();
    is_copy::<*mut String>();

    // immutable reference (不可变引用) 实现了 Copy trait
    // 不可变引用（&T）是指向某个数据的引用，它不允许通过该引用修改数据
    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();

    // 如果一个数组或元组中的所有值都实现了 Copy trait，那么这个数组或元组本身也会实现 Copy trait。
    is_copy::<[i32; 3]>();
    is_copy::<(&str, &str)>();
}

fn types_not_impl_copy_trait() {
    // unsize 或 动态长度类型 不实现 Copy trait
    is_copy::<str>();
    is_copy::<[u8]>();
    is_copy::<Vec<u8>>();
    is_copy::<String>();

    // mutable reference (可变引用) 不实现 Copy trait
    is_copy::<&mut String>();

    // 如果一个数组或元组中的有值没有实现 Copy trait，那么这个数组或元组不实现 Copy trait。
    is_copy::<(String, u32)>();
}

fn main() {
    // 原生类型，包括函数、不可变引用和裸指针都实现了 Copy trait
    // 数据和元祖，如果内部的数据结构实现了 Copy trait，那么他们也会实现 Copy trait
    // 可变引用没有实现 Copy trait
    // 非固定大小的数据结构没有实现 Copy trait
    is_copy::<bool>();
    is_copy::<i32>();
    types_impl_copy_trait();
    types_not_impl_copy_trait();
}
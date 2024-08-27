use std::fmt::Display;

fn main() {
    /*
        生命周期标注语法

        &i32                // 引用类型
        &'a i32             // 带有生命周期的引用类型
        &'a mut i32         // 带有生命周期的可变引用类型
     */

     func_lifecycle();
     struct_lifecycle();
     method_lifecycle();
     static_lifecycle();
     sample();
}

// 函数的生命周期
// 标记生命周期用来将函数的多个引用参数和返回值的作用域关联到一起。
// 一旦关联到一起，Rust 就拥有充分的信息来确保代码是内存安全的。
// 生命周期并不会改变任何引用的实际作用域。
fn func_lifecycle() {
    // 此处的生命周期标注仅用于说明，x 和 y 的生命周期和 'a 一样，至于谁活得更久无法得知
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let str1 = String::from("abcd");
    let str2 = "xyz";
    let result = longest(str1.as_str(), str2);
    assert_eq!(result, "abcd");
}

// 结构体的生命周期
fn struct_lifecycle() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { 
        part: first_sentence 
    };
    assert_eq!(i.part, first_sentence);
    // dbg!(novel);             // 错误，novel 的生命周期已经结束   
    dbg!(first_sentence);       // 而 first_sentence 的生命周期和 i 一样，因此仍然有效
}

// 生命周期消除
/*
    三条消除规则：
        1. 每个引用参数都会获得独自的生命周期
        2. 若只有一个输入生命周期（函数参数只有一个引用类型），那么该生命周期会被赋给所有输出生命周期
        3. 若有多个输入生命周期，且其中一个是 &self 或 &mut self，那么该生命周期会被赋给所有输出生命周期
*/

// 方法中的生命周期
fn method_lifecycle() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    // impl 中必须使用结构体的完整名称，包括 <'a> ，因为生命周期标注也是结构体的一部分
    // 方法签名中，往往不需要标注生命周期，得益于第一和三条消除规则
    impl<'a: 'c, 'c> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            1
        }

        // 返回值的生命周期和 self 一样，因此可以省略生命周期标注
        // fn announce_and_return_part(&self, announcement: &str) -> &'a str {
        //     println!("Attention please: {}", announcement);
        //     self.part
        // }
        fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }

        // 在 impl<'a: 'c, 'c> 中，'a: 'c 为生命周期约束语法，表示 'a 的生命周期至少要和 'c 一样长
        fn announce_and_return_part1(&'a self, announcement: &'c str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }

        // 通过 where 语法约束，表示 'a 的生命周期至少要和 'd 一样长
        fn announce_and_return_part2<'d>(&'a self, announcement: &'d str) -> &'d str 
        where 
            'a: 'd
        {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    let i = ImportantExcerpt { part: "hello" };
    assert_eq!(i.level(), 1);
    assert_eq!(i.part, "hello");
    assert_eq!(i.announce_and_return_part("whatever"), "hello");
    assert_eq!(i.announce_and_return_part1("whatever"), "hello");
    assert_eq!(i.announce_and_return_part2("whatever"), "hello");

}

// 静态生命周期
fn static_lifecycle() {
    // 生命周期 'static 意味着能和程序活得一样久，例如字符串字面量和特征对象
    // 实在遇到解决不了的生命周期标注问题，可以尝试 T: 'static 
    let s: &'static str = "I have a static lifetime.";
    dbg!(s);
}

fn sample() {
    fn longest<'a, T> (
        x: &'a str, 
        y: &'a str, 
        ann: T
    ) -> &'a str 
    where 
        T: Display {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let str1 = String::from("long string is long");
    let str2 = String::from("long");
    let result = longest(str1.as_str(), str2.as_str(), "whatever");
    assert_eq!(result, "long string is long");
}
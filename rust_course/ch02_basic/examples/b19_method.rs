fn main() {
    define_method();
    define_method_and_field();
    define_multi_impl();
    define_method_for_enum();
}

// 定义方法
fn define_method() {
    struct Circle {
        r: f64,
    }

    impl Circle {
        // Circle 的关联函数，因为它的第一个参数不是 self,且 new 不是关键字
        // 这种方法往往用于初始化当前构造体实例
        // Rust 使用 new 作为构造器名称是约定俗成的
        // 定义在 impl 中且没有 self 的函数称为关联函数，只能通过 Circle::new 的方式调用
        fn new(r: f64) -> Circle {
            Circle {
                r,
            }
        }

        // Circle 的实例方法，&self 表示借用当前的 Circle 结构体实例
        // self         表示 Circle 的所有权转移到该方法中
        // &self        表示该方法对 Circle 的不可变借用
        // &mut self    表示该方法对 Circle 的可变借用
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.r * self.r)
        }
    }

    let c = Circle::new(10.0);
    assert_eq!(314.1592653589793, c.area());
}

fn define_method_and_field() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn width(&self) -> bool {
            self.width > 0
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, rect: &Rectangle) -> bool {
            self.width > rect.width && self.height > rect.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // 调用方法
    if (&rect1).width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // Rust 可以自动解引用
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let area = rect1.area();
    let can_hold = rect1.can_hold(&Rectangle {
        width: 10,
        height: 10,
    });
    dbg!(area, can_hold);
}

// 多个 impl 定义
fn define_multi_impl() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, rect: &Rectangle) -> bool {
            self.width > rect.width && self.height > rect.height
        }
    }

    let rect1 = Rectangle {
        width: 5,
        height: 50,
    };
    let area = rect1.area();
    let can_hold = rect1.can_hold(&Rectangle {
        width: 10,
        height: 10,
    });
    dbg!(area, can_hold);
}

// 为枚举实现方法
fn define_method_for_enum() {
    #[derive(Debug)]
    enum Message {
        Quit,
    }

    impl Message {
        fn call(&self) {
            println!("{:?} called", self);
        }
    }

    let m = Message::Quit;
    m.call();
}
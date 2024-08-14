use core::fmt;
use std::{fmt::Display, ops::Add};

fn main() {
    relate_type();
    default_generic_type();
    same_method_name_in_diff_trait();
    trait_constraint();
    newtype();
}

// 关联类型
// 关联类型是在特征定义的语句块，声明一个自定义类型，这样就可以在特征的方法前面中使用该类型了
fn relate_type() {

    struct Counter (
        i32
    );

    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    } 

    impl Iterator for Counter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 < 5 {
                self.0 += 1;
                Some(self.0)
            } else {
                None
            }
        }
    }

    let mut c = Counter(0);
    dbg!(c.next());
}

// 默认泛型类型参数
fn default_generic_type() {
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, rhs: Self) -> Point {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    assert_eq!(Point { x: 1, y: 2 } + Point { x: 2, y: 3 }, Point { x: 3, y: 5 });

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, rhs: Meters) -> Millimeters {
            Millimeters(self.0 + (rhs.0 * 1000))
        }
    }

    assert_eq!((Millimeters(1000) + Meters(1)).0, Millimeters(2000).0);
}

// 调用同名方法
fn same_method_name_in_diff_trait() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    trait Animal {
       fn baby_name() -> String; 
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    dbg!(Dog::baby_name());
    dbg!(<Dog as Animal>::baby_name());
}

// 特征定义中的特征约束
fn trait_constraint() {
    trait OutlinePrint: Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {}
    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let p = Point { x: 1, y: 2 };
    p.outline_print();
}

// 在外部类型上实现外部特征
// 为一个元组结构体创建新类型，该元组结构体封装有一个字段，该字段就是希望实现特征的具体类型
fn newtype() {

    #[derive(Debug)]
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    dbg!(w);
}
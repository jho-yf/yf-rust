use std::{char, fmt::Display};


// 特征定义了一组可以被共享的行为，只要实现了特征，就可以使用这组行为
fn main() {
    sample();
    trait_params();
    trait_bound();
    multiple_bounds();
    where_constraint();
    impl_constraint();
    return_impl_trait();
    largest_demo();
    custom_type_impl();
    custom_type_print();
}

// 定义特征，实现特征
// 孤儿规则
fn sample() {
    pub trait Summary {
        fn summarize(&self) -> String;

        // 默认实现
        fn default_summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct Post {
        pub title: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("title: {}, by {}", self.title, self.author)
        }        
    }

    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{} posted: {}", self.username, self.content)
        }

        fn default_summarize(&self) -> String {
            String::from("(override...)")
        }
    }

    let post = Post {
        title: "Rust".to_string(),
        author: "Rust".to_string(),
        content: "Rust is awesome".to_string(),
    };
    println!("{}", post.summarize());
    println!("{}", post.default_summarize());

    let weibo = Weibo {
        username: "Rust".to_string(),
        content: "Rust is awesome".to_string(),
    };
    println!("{}", weibo.summarize());
    println!("{}", weibo.default_summarize())
}

// 使用特征作为函数参数
fn trait_params() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    impl Summary for i32 {
        
        fn summarize(&self) -> String {
            format!("summary: {}", self)
        }

    }

    let item: i32 = 1;
    notify(&item);
}

// 特征约束
fn trait_bound() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    impl Summary for String {
        fn summarize(&self) -> String {
            format!("summary: {}", self)
        }
    }

    let item: String = "hello".to_string();
    notify(&item);
}

// 多重约束
fn multiple_bounds() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub trait Enrich {
        fn enrich(&self) -> String;
    }

    pub fn notify<T: Summary + Enrich>(item: &T) {
        println!("notify: {}, enrich: {}", item.summarize(), item.enrich());
    } 

    impl Summary for String {
        fn summarize(&self) -> String {
            format!("summarize: {}", self)
        }
    }

    impl Enrich for String {
        fn enrich(&self) -> String {
            format!("[{}]", self)
        }
    }

    let item: String = "hello".to_string();
    notify(&item);
}

// Where 约束
fn where_constraint() {
    fn some_func<T, U>(t: &T, u: U) -> U
        where T: Display + Clone, U: Clone {
        println!("t = {}", t);
        u.clone()
    }

    let item: String = "hello".to_string();
    let item2: i32 = 1;
    let item3: i32 = some_func(&item, item2);
    assert_eq!(item2, item3);
}

// 使用特征约束有条件地实现方法或特征
fn impl_constraint() {
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T>  {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // T 类型必须同时实现 Display 和 PartialOrd 特征，才能调用 cmp_display 方法
    impl <T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    
    let p = Pair::new(1_i32, 2_i32);
    p.cmp_display();

}

// 函数返回中的 impl trait
fn return_impl_trait() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    impl Summary for String {
        fn summarize(&self) -> String {
            format!("summary string: {}", self)
        }
    }

    pub fn returns_summary() -> impl Summary {
        String::from("hello")
    }

    let item = returns_summary();
    println!("{}", item.summarize());
}

fn largest_demo() {
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {

            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let the_largest = largest(&number_list);
    dbg!(the_largest);

    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
    let the_largest = largest(&char_list);
    dbg!(the_largest);
}

// 自定义类型实现 + 操作
fn custom_type_impl() {
    use std::ops::Add;

    // 为 Point 结构体派生 Debug 特征，用于格式化输出
    #[derive(Debug)]
    struct Point<T: Add<T,  Output = T>> {
        x: T,
        y: T,
    }

    impl<T: Add<T, Output = T>> Add for Point<T> {
        type Output = Point<T>;

        fn add(self, other: Point<T>) -> Point<T> {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
        a + b
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    println!("p1 + p2 = {:?}", p1 + p2);

    let p3 = Point { x: 1.1, y: 2.8 };
    let p4 = Point { x: 3.0, y: 4.4 };
    println!("p3 + p4 = {:?}", add(p3, p4));
}

// 自定义类型的打印输出
fn custom_type_print() {
    #![allow(dead_code)]

    #[derive(Debug, PartialEq)]
    enum FileState {
        Open,
        Closed,
    }

    #[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
        state: FileState,
    }

    use std::fmt;

    impl Display for FileState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                FileState::Open => write!(f, "Open"),
                FileState::Closed => write!(f, "Closed"),
            }
        }
    }

    impl Display for File {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "<{} ({})>", self.name, self.state)
        }
    }

    impl File {
        fn new(name: &str) -> File {
            File {
                name: name.to_string(),
                data: Vec::new(),
                state: FileState::Closed,
            }
        }
    }

    let f = File::new("f6.txt");
    println!("{:?}", f);
    println!("{}", f);
}
fn main() {
    def_trait_object();
    sample();
    self_();
}

// 特征对象定义
fn def_trait_object() {
    pub trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }

    // 参数是 Box<dyn Draw> 形式的特征对象，该特征对象是通过 Box::new(x) 的方式创建的
    // dyn 关键字只用于特征对象的类型声明上，在创建时无需使用 dyn
    fn draw1(x: Box<dyn Draw>) {
        x.draw();
    }

    // 参数是 &dyn Draw 形式的特征对象，该特征对象是通过 &x 的方式创建的
    // dyn 关键字只用于特征对象的类型声明上，在创建时无需使用 dyn
    fn draw2(x: &dyn Draw) {
        x.draw();
    }

    // x 和 y 的类型实现了 Draw 特征, Box<T> 可以在函数调用时隐式地转换为 Box<dyn Draw>
    let x = 1.1f64;
    let y = 1u8;

    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，该指针指向的数据被放置与堆上
    draw1(Box::new(x));
    // 基于 y 的值创建一个 Box<f64> 类型的智能指针，该指针指向的数据被放置与堆上
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
}

// sample
fn sample() {
    pub trait Draw {
        fn draw(&self) -> String;
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                println!("{}", component.draw());
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) -> String {
            format!("draw button: {}, width: {}, height: {}", 
                self.label, self.width, self.height)
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) -> String {
            format!("draw select box: , width: {}, height: {}, options: {:?}", 
                self.width, self.height, self.options)
        }
    }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ],
    };

    screen.run();
}

// Self 和 self
// self 指代当前实例对象
// Self 指代特征或者方法所在类型的别名
fn self_() {
    trait Draw {
        fn draw(&self) -> Self;
    }

    #[derive(Clone, Debug)]
    struct Button;

    impl Draw for Button {
        fn draw(&self) -> Self {
            return self.clone();
        }
    }

    let btn = Button;
    let btn2 = btn.draw();
    println!("btn: {:?}, btn2: {:?}", btn, btn2)
}
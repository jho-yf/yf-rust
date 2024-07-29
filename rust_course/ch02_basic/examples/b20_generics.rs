fn main() {
    sample();
    generic_struct();
    generic_enum();
    generic_method();
    const_generic();
}

// 泛型，多态
fn sample() {
    fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }

    println!("add i8: {}", add(1i8, 2i8));
}

// 在结构体中使用泛型
fn generic_struct() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };

    println!("p1: {:?}", p1);
    println!("p1.x: {:?}, p1.y: {:?}", p1.x, p1.y);
    println!("p2: {:?}", p2);
    println!("p2.x: {:?}, p2.y: {:?}", p2.x, p2.y);
}

// 在枚举中使用泛型
fn generic_enum() {
    #[derive(Debug)]
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let r1: Result<i32, ()>  = Result::Ok(1);
    let r2: Result<(), String>  = Result::Err("error".to_string());
    dbg!(r1, r2);
}

// 方法中使用泛型
fn generic_method() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &T {
            &self.y
        }
    }

    let p = Point { x: 1, y: 2 };
    dbg!(p.x(), p.y());

    // 为具体泛型类型实现方法
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            ((self.x * self.x) + (self.y * self.y)).sqrt()
        }
    }

    let p = Point { x: 1.0, y: 2.0 };
    dbg!(p.distance_from_origin());
}

// const 泛型
fn const_generic() {
    fn display_arr<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }

    let arr = [1, 2, 3];
    display_arr(arr);

    let arr = [1.0, 2.0, 3.0, 4.0];
    display_arr(arr);
}
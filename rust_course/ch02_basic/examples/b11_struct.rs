fn main() {
    struct_demo();

    struct_memory();

    tuple_struct();

    unit_like_struct();
}

// 结构体语法
fn struct_demo() {
    let user1 = build_user("jho".to_string(), "jho@jho.com".to_string());

    let user2 = User {
        username: String::from("jho2"),
        // 此处 user1.email 的所有权会被转移到 user2.email 中
        ..user1
    };
    // 由于 user1.email 已经被转移了所有权，所以下面这两行代码会报错
    // println!("user1={}", user1);
    // println!("user1.email={}", user1.email);
    // 但是这行代码不会报错
    println!("user1={}", user1.active);

    dbg!(user2.active, user2.username, user2.email, user2.sign_in_count);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// 结构体内存排列
fn struct_memory() {
    #[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
    }

    let f1 = File {
        name: String::from("f1"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_len = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_len);
}

// 元组结构体
fn tuple_struct() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 1, 2);
    let origin = Point(0, 1, 2);
    dbg!(black.0, black.1, black.2, black);
    dbg!(origin.0, origin.1, origin.2, origin);
}

// 单元结构体
// 当定义一个类型，但不关心该类型的内容，只关心他的行为时，可以使用单元结构体。
fn unit_like_struct() {
    struct MyPrinter;

    trait SomeTrait {
        fn print(&self);
    }

    impl SomeTrait for MyPrinter {
        fn print(&self) {
            println!("Hello, rust!");
        }
    }

    let subject = MyPrinter;
    subject.print();
}
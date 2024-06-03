/*
    在定义数据结构的时候，我们一般会加入修饰，为数据结构引入一些额外的行为。
    在 Rust 中，数据的行为通过 trait 来定义，trait可以简单地认为定义了数据结构可以实现的接口，类似java的interface。

    #[derive(Debug, Copy, Clone)]
    Debug：为结构体实现Debug trait，可以打印出结构体的信息
    Copy：为结构体实现Copy trait，让数据结构可以在参数传递过程时按字节拷贝
    Clone：为结构体实现Clone trait，让数据结构可以被复制
*/

// 枚举类型
#[derive(Debug)]
#[allow(dead_code)]
enum Gender {
    Unspecified = 0,
    Male = 1,
    Female = 2,
}

// struct的特殊形式：元组结构体
// 它的域都是匿名的，可以使用索引访问，常用于构造简单的结构体
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
struct TopicId(u64);

// 标准结构体，可以把任何类型组合在结构体中使用
#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Topic {
    id: TopicId,
    title: String,
    owner: UserId
}

// 标准的标签联合体：定义聊天室中可能发生的事件
// Join: 用户加入聊天室
// Leave: 用户离开聊天室
// Message: 用户发送消息
#[derive(Debug)]
#[allow(dead_code)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn main() {
    let tom = User {
        id: UserId(1),
        name: "Tom".into(),
        gender: Gender::Male,
    };
    let alice = User {
        id: UserId(2),
        name: "Alice".into(),
        gender: Gender::Female,
    };

    let rust_topic = Topic {
        id: TopicId(1),
        title: "Rust".into(),
        owner: UserId(1)
    };

    let event1 = Event::Join((tom.id, rust_topic.id));
    let event2 = Event::Join((alice.id, rust_topic.id));
    let event3 = Event::Message((tom.id, rust_topic.id, "Hello, Alice!".into()));

    println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);
}

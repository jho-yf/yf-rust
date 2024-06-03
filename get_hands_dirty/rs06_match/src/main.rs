#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

// 定义聊天室可能发生的事件
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn process_event(event: Event) {
    match event {
        Event::Join((uid, _tid)) => println!("User {:?} joined topic {:?}", uid, _tid),
        Event::Leave((uid, tid)) => println!("User {:?} left topic {:?}", uid, tid),
        Event::Message((_, _, message)) => println!("broadcast: {:?}", message)
    }
}

fn process_message(event: &Event) {
    if let Event::Message((_, _, message)) = event {
        println!("broadcast: {}", message)
    }
}

fn main() {
    let join_event = Event::Join((UserId(1), TopicId(1)));
    process_event(join_event);
    let leave_event = Event::Leave((UserId(1), TopicId(1)));
    process_event(leave_event);
    let message_event = Event::Message((UserId(1), TopicId(1), "Hello!".into()));
    process_event(message_event);

    let join_event1 = Event::Join((UserId(1), TopicId(1)));
    process_message(&join_event1);
    let leave_event1 = Event::Join((UserId(1), TopicId(1)));
    process_message(&leave_event1);
    let message_event1 = Event::Message((UserId(1), TopicId(1), "Hello!".into()));
    process_message(&message_event1);
}

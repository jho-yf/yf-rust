// import commonly used items from the prelude:
use rand::prelude::*;

fn main() {
    // We can use random() immediately. It can produce values of many common types:
    let x: u8 = random();
    println!("{}", x);
}

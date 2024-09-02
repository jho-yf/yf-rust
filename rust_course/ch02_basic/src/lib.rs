// 在同名文件中加载该模块的内容
mod front_of_house;

pub use crate::front_of_house::hosting;

mod serving {
    fn take_order() {
        println!("Took order");
    }
    fn serve_order() {
        println!("Served order");
    }
    fn take_payment() {
        println!("Took payment");
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    // crate ：使用绝对路径引用模块
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // 使用相对路径应用模块
    front_of_house::hosting::add_to_waitlist();

    // 使用 use 简化引用
    hosting::add_to_waitlist();
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 使用 super 引用父模块
        super::serve_order();
    }

    pub fn cook_order() {
        println!("Cooked order");
    }
}

fn serve_order() {
    // 使用 self 引用当前模块
    self::back_of_house::cook_order();
}
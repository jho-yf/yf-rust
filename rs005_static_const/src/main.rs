static MY_STATiC: i32 = 100;
static mut MY_STATiC_MUT: i32 = 100;

fn main() {
    const SECOND_HOUR: usize = 3_600;
    const SECOND_DAY: usize = 24 * SECOND_HOUR;

    {
        const SE: usize = 1_000;
        println!("{SE}")
    }

    println!("{} seconds in a day", SECOND_DAY);
    println!("{MY_STATiC}");

    unsafe {
        MY_STATiC_MUT = 200;
        println!("{MY_STATiC_MUT}");
    }
}

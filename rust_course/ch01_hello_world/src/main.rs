fn main() {
    greet_world();
    print_penguin_data();
}

fn greet_world() {
    let chinese = "你好";
    let english = "Hello";

    let reginons = [chinese, english];

    for region in reginons {
        println!("{region}");
    }
}

fn print_penguin_data() {
    let penguin_data = "\
        common name,length (cm)
        Little penguin,33
        Yellow-eyed penguin,65
        Fiordland penguin,60
        Invalid,data
        ";
    let records = penguin_data.lines();

    for (i, record) in records.enumerate()  {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // 声明 fields 变量，类型为 Vec 
        // Vec 动态数组
        // <_> 表示 Vec 中的元素类型由编译器自行推断
        let fields: Vec<_> = record.split(",")
            .map(|field| field.trim())
            .collect();
        if cfg!(debug_assertions) {
            // 输出到标准错误输出
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];

        // 尝试把 fields[1] 转换为 f32 类型,如果成功则将值赋给 length 变量
        if let Ok(length) = fields[1].parse::<f32>(){
            println!("{}, {}cm", name, length);
        }
    }
}

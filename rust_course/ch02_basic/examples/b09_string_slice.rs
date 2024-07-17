/*
    Rust 中的字符是 Unicode 类型，每个字符占据4个字节
    字符串是 UTF-8 编码，字符串中的字符占的字节数是变化的，1 - 4

    Rust 语言级别的字符串类型是 str ，通常以引用类型出现：&str
    str 类型称字符串字面量，是硬编码进可执行文件的，一次无法被修改，它是个切片
    String 类型则是一个可增长、可改变且具有所有权的 UTF-8 编码字符串

    字符串的底层数据存储格式实际上是 u8 字节数组
*/

fn main() {
    slice();

    arr_slice();

    convert_between_string_and_str();

    string_handle();

    escaping();

    utf8_string_handle();
}

// 切片
fn slice() {
    let s = String::from("hello world");

    // [开始索引..终止索引]
    let hello = &s[0..5];
    let world = &s[6..11];
    assert_eq!(hello, "hello");
    assert_eq!(world, "world");

    let hello = &s[..5];
    let world = &s[6..];
    assert_eq!(hello, "hello");
    assert_eq!(world, "world");

    // 截取完整的字符串切片
    let hello_world = &s[..];
    assert_eq!(hello_world, "hello world");
    let len = s.len();
    let hello_world = &s[..len];
    assert_eq!(hello_world, "hello world");
}


// 数组切片
fn arr_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// String 与 &str 的转换
fn convert_between_string_and_str() {
    // &str -> String
    #![allow(unused_variables)]
    let s = String::from("hello");

    // String -> &str : deref 隐式强制转换
    let str1: &str = &s;
    let str2: &str = &s[..];
    let str3: &str = s.as_str();
}

fn string_handle() {
    // 追加
    let mut s = String::from("hello ");
    s.push_str("rust");  
    println!("追加字符串 push_str() -> {}", s);
    s.push('!');
    println!("追加字符 push() -> {}", s);

    // 插入
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);

    // 替换
    let s = String::from("i like rust, rust is nice!");
    let new_s = s.replace("rust", "RUST");
    dbg!(new_s);
    let new_s = s.replacen("rust", "RUST", 1);
    dbg!(new_s);
    let mut s = String::from("i like rust, rust is nice!");
    s.replace_range(7..8, "R");
    dbg!(s);

    // 删除
    // pop 删除并返回字符串的最后一个字符
    let mut string_pop = String::from("rust pop 中文！");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1, p2, string_pop);
    
    // remove 删除并返回字符串中指定位置的字符
    let mut string_remove = String::from("测试remove方法");
    println!(
        // 一共占 18 个字符，其中字母占1个字符，汉字占3个字符
        "string_remove 占 {} 个字符", 
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第二个汉字
    let c2 = string_remove.remove(3);
    // 删除第一个汉字
    let c1 = string_remove.remove(0);
    dbg!(c1, c2, string_remove);
    
    // truncate 删除字符串中指定位置之后的字符
    let mut string_truncate = String::from("测试truncate方法");
    string_truncate.truncate(3);
    dbg!(string_truncate);

    // clear 清空字符串
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    // concatenate 字符串连接
    // 使用 + 或者 += 连接字符串
    let str_append = String::from("hello ");
    let str_rust = String::from("rust");
    // $str_rust 会自动解引用为 &str
    let concat_result = str_append + &str_rust;
    let mut concat_result = concat_result + "!";
    concat_result += "!!!!";
    dbg!(concat_result);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let concat_result = s1 + "-" + &s2 + "-" + &s3;
    dbg!(concat_result);

    let s1 = String::from("tic");
    let format_result = format!("{}-{}-{}", s1, s2, s3);
    dbg!(format_result);
}

// 字符串转义
fn escaping() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    // 在某些情况下，可能希望保持字符串的原样
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}

// 操作 utf8 字符串
fn utf8_string_handle() {
    // 字符
    for c in "普通话".chars() {
        println!("{}", c);
    }

    // 字节
    for b in "普通话".bytes() {
        println!("{}", b);
    }
}
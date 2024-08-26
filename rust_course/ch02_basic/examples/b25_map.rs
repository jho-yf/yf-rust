use std::collections::HashMap;

fn main() {
    map_creation();
    ownership();
    query_map();
    update_map();
}

fn map_creation() {
    // new 方法创建
    let mut my_gems = HashMap::new();
    my_gems.insert("ruby", 1);
    my_gems.insert("sapphire", 2);
    my_gems.insert("emerald", 3);
    dbg!(my_gems);

    // 创建指定大小的 HashMap
    let mut my_gems = HashMap::with_capacity(10);
    my_gems.insert("ruby", 1);
    dbg!(my_gems);

    // 使用迭代器和 collect 方法创建
    let teams = vec![
        ("China".to_string(), 1000),
        ("USA".to_string(), 200),
        ("Japan".to_string(), 300),
    ];
    let teams_map = teams.into_iter().collect::<HashMap<_, _>>();
    dbg!(teams_map);

    let teams = vec![
        ("China".to_string(), 1000),
        ("USA".to_string(), 200),
        ("Japan".to_string(), 300),
    ];
    let teams_map: HashMap<_, _> = teams.into_iter().collect();
    dbg!(teams_map);
}

// 所有权转移
// 若类型实现 Copy 特征，该类型会被复制进 HashMap ，因此无所谓所有权
// 若类型没有实现 Copy 特征，则所有权会被转移给 HashMap
fn ownership() {
    let name = String::from("Tom");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(name, age);

    // name 所有权已转移给 HashMap
    // dbg!(name);

    // age 所有权未被转移
    dbg!(age);
}

fn query_map() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 1000);
    scores.insert("Yellow".to_string(), 200);

    let team_name = String::from("Blue");
    let score = scores[&team_name];
    assert_eq!(score, 1000);

    let score = scores.get(&team_name);
    assert_eq!(score, Some(&1000));

    let score = scores.get(&team_name).copied().unwrap_or(0);
    assert_eq!(score, 1000_i32);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn update_map() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 1000);

    // 覆盖并返回旧值
    let old = scores.insert("Blue".to_string(), 2000);
    assert_eq!(old, Some(1000));

    // 查询值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&2000));

    // 查询值，若不存在则插入新值
    let v = scores.entry("Yellow".to_string()).or_insert(100);
    assert_eq!(*v, 100);
    let v = scores.entry("Yellow".to_string()).or_insert(500);
    assert_eq!(*v, 100);

    // 在已有值的基础上更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    dbg!(map);
}
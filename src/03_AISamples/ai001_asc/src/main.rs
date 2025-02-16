//////////////////////////////
/// 
/// AIサンプル　構造体のソート
/// 
/// /////////////////////////

//#[derive(Debug)]
struct Person {
    pub name: String,
    pub age: u32,
}

fn main() {
    let mut people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];

    // 年齢を基準に昇順ソート
    people.sort_by(|a, b| a.age.cmp(&b.age));

    for human in people
    {
        println!("{}/{}", human.name,human.age);
    }
}

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let a = 5;
    let foo_value = foo(a);
    println!("{}", foo_value);

    println!("{:?}", test_lambda());
    test_map();
}

fn  foo(x: i32) -> i32 {
    x + 1
}

fn test_lambda()  {
    let vec = vec![1, 2, 3, 4, 5];
    let result = vec.iter().fold(0, |acc, x| acc + x);
    println!("{}", result);
    let sum: u8 = vec.iter().sum();
    println!("{}", sum);
    let sum: u8 = vec.iter().map(|x| x + 1).sum();
    println!("{}", sum);
    let a = vec.first()
        .map(|x| x + 1)
        .unwrap_or(0);
    println!("{}", a);
}

fn test_map() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    println!("{:?}", map);
    map.entry("c").or_insert(3);
    println!("{:?}", map);
    let entry_value = map.entry("a");
    println!("{:?}", entry_value);
    let key = entry_value.key();
    println!("the key is {}", key);
    let mut letters = HashMap::new();

    for ch in "a short treatise on fungi".chars() {
        letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }
    println!("{:?}", letters);
}

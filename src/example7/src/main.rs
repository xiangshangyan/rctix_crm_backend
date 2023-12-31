use std::collections::HashMap;
use std::mem::size_of_val;

fn main() {
    let c1 = || println!("Hello, world!");

    let c2 = |i: i32| println!("the i32 is {}", i);

    let name = String::from("Peter");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("yan", "yuchuan");
    let c3 = || println!("Hello, {}!", name);
    let c4 = move || println!("Hello, {}, {:?}!", name1, table);
    let name2 = name.clone();
    let c5 = move || {
        let x = 1;
        let name3 = String::from("lindsey");
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };

    println!("c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}", size_of_val(&c1), size_of_val(&c2), size_of_val(&c3), size_of_val(&c4), size_of_val(&c5), size_of_val(&main), );

    test_fn_once();
}

fn test_fn_once() {
    let name = String::from("Tyr");    // 这个闭包啥也不干，只是把捕获的参数返回去
    let c = move |greeting: String| (greeting, name);
    let result = c("hello".to_string());
    println!("result: {:?}", result);    // 无法再次调用
    // let result = c("hi".to_string());
}

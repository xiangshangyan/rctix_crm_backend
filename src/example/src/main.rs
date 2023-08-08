fn main() {
    println!("Hello, world!");
    test_enum();
    test_fn_program();
    test_iter();
}

enum OperateEnum {
    Add,
    Delete,
    Update,
    Query,
}

fn test_enum() {
    let mut operate_enum = OperateEnum::Delete;
    match &operate_enum {
        OperateEnum::Add => { println!("Add") }
        OperateEnum::Delete => { println!("Delete") }
        OperateEnum::Update => { println!("Update") }
        OperateEnum::Query => { println!("Query") }
    }

    operate_enum = OperateEnum::Add;
    match operate_enum {
        OperateEnum::Delete => println!("delete"),
        // 其他情况
        _ => println!("other"),
    }
}

fn test_fn_program() {
    let is_even = |x| {
        0 == x % 2
    };
    let num = 10;
    let result = is_even(num);
    println!("the num is even {}", result);
}

fn test_iter() {
    let mut vec_i32 = vec![23, 22, 15];
    // 只读可重入迭代器
    for x in vec_i32.iter() {
        println!("the x is {}", x);
    }
    println!("{:?}", vec_i32);// [23, 22, 15]

    // 改变了原来vec中的值
    for x in vec_i32.iter_mut() {
        *x += 1;
    }
    println!("{:?}", vec_i32);// [24, 23, 16]

    // 消耗掉数值
    for x in vec_i32.into_iter() {
        println!("x {}", x);
    }
    // println!("{:?}", vec_i32);// 报错，已被迭代器消费掉

    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // 可以被2整除的集合
    let vec1: Vec<_> = vec.iter().filter(|x| *x % 2 == 0).collect();
    println!("vec1 {:?}", vec1);
    // 可以被3整除的数据之和
    let sum = vec.iter().filter(|y| *y % 3 == 0).sum::<i32>();
    println!("the sum is {}", sum);

    // 对vec进行排序
    let mut vec2 = vec![23, 879, 98];
    vec2.sort();
    println!("the vec2 is {:?}", vec2);
}

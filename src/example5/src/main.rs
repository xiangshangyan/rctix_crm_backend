fn main() {
    println!("Hello, world!");
    let a = 5;
    let foo_value = foo(a);
    println!("{}", foo_value);

    println!("{:?}", test_lambda());
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

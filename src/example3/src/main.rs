fn main() {
    println!("Hello, world!");

    let str1 = "rust";
    let str2 = "trus";

    println!("{}", anagram_solution2(str1, str2));
}

// 比较两个字符串是否是乱序字符串
fn anagram_solution2(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();
    for c in s1.chars() {
        vec_a.push(c);
    }
    for c in s2.chars() {
        vec_b.push(c);
    }
    vec_a.sort();
    println!("{:?}", vec_a);
    vec_b.sort();
    println!("{:?}", vec_b);
    vec_a == vec_b
}

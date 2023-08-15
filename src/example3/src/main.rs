fn main() {
    println!("Hello, world!");

    let str1 = "rust";
    let str2 = "trus1";

    println!("{}", anagram_solution2(str1, str2));
    println!("{}", anagram_solution3(str1, str2));

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

// 比较两个字符串是否是乱序字符串
fn anagram_solution3(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    // 大小为26的集合，用于将字符映射为ASCII的值
    let mut c1 = [0; 26];
    let mut c2 = [0; 26];
    for c in s1.chars() {
        c1[c as usize - 'a' as usize] += 1;
    }
    for c in s2.chars() {
        c2[c as usize - 'a' as usize] += 1;
    }
    for i in 0..26 {
        if c1[i] != c2[i] {
            return false;
        }
    }
    true
}

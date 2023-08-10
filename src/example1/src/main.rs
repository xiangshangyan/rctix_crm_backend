fn main() {
    println!("Hello, world!");
    let vec = vec![1, 2];
    // test_fn_once(|z| z == vec.len());
    test_fn_once_copy(|z| z == vec.len());
}

struct Cacher<T> where T: Fn(u32) -> u32 {
    query: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(query: T) -> Cacher<T> {
        Cacher {
            query,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
            Some(v) => { v }
        }
    }
}


fn test_fn_once<F>(func: F) where F: FnOnce(usize) -> bool {
    println!("{}", func(2));
    // println!("{}", func(1));// 到此处生命周期已结束，会报错
}

/*
 实现Copy可以执行成功
 */
fn test_fn_once_copy<F>(func: F) where F: FnOnce(usize) -> bool + Copy {
    println!("{}", func(2)); // true
    println!("{}", func(1)); // false
}

use std::vec::IntoIter;

fn main() {
    println!("Hello, world!");

    let mut stack = Stack::new();
    let is_empty = stack.is_empty();
    println!("is_empty: {}", is_empty);
    stack.push(1);
    stack.push(2);
    stack.push(3);

    let is_empty = stack.is_empty();
    println!("is_empty: {}", is_empty);

    let top = stack.pop();
    println!("top: {:?}", top);

    let len = stack.len();
    println!("len: {}", len);

    let peek = stack.peek();
    println!("peek: {:?}", peek);

    let peek_mut = stack.peek_mut();
    println!("peek_mut: {:?}", peek_mut);
    if let Some(top) = peek_mut {
        *top = 10;
    }
    println!("stack: {:?}", stack);

    // let sum = stack.into_iter().sum::<i32>();
    // println!("sum: {:?}", sum);

    let iter_sum:i32 = stack.iter().sum();
    println!("iter_sum: {:?}", iter_sum);

    let mut addend = 0;
    for x in stack.iter_mut() {
        *x += 1;
        addend += 1;
    }
    println!("addend: {:?}", addend);
    println!("stack: {:?}", stack);
}

#[derive(Debug)]
struct Stack<T> {
    // 栈的大小
    size: usize,
    // 栈数据
    data: Vec<T>,
}

impl<T> Stack<T> {
    // 构造函数，初始化栈
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.data.clear();
        self.size = 0;
    }

    fn push(&mut self, elem: T) {
        self.data.push(elem);
        self.size += 1;
    }

    // 弹出栈顶元素
    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            Some(self.data.pop().unwrap())
        }
    }

    fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            Some(&self.data[self.size - 1])
        }
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            None
        } else {
            Some(&mut self.data[self.size - 1])
        }
    }

    fn into_iter(self) -> MyInToIter<T> {
        MyInToIter(self)
    }

    fn iter(&self) -> MyIter<T> {
        let mut iterator = MyIter {
            stack: Vec::new()
        };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    fn iter_mut(&mut self) -> MyIterMut<T> {
        let mut iterator = MyIterMut {
            stack: Vec::new()
        };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

struct MyInToIter<T> (Stack<T>);
// 实现迭代器trait
impl<T> Iterator for MyInToIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct MyIter<'a, T: 'a> {
    stack: Vec<&'a T>
}
impl <'a, T> Iterator for MyIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            self.stack.pop()
        }
    }
}

struct MyIterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>
}
impl <'a, T> Iterator for MyIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            self.stack.pop()
        }
    }
}

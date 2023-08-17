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

    let mut peek_mut = stack.peek_mut();
    println!("peek_mut: {:?}", peek_mut);
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
}

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use tokio::time::Duration;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let async_task_handle = tokio::spawn(test_async());
    println!("11111111111111");
    // 会休眠两秒钟后继续执行
    async_task_handle.await.unwrap();
    println!("22222222222222");
    test_rc();
    test_struct_cell();
    test_ref_cell();
}

async fn test_async() {
    println!("Async task start...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Async task end...")
}

// async fn test_sync() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];
//     for _ in 0..10 {
//         let counter = counter.clone();
//         let handle = tokio::spawn(async move {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.await.unwrap();
//     }
//     println!("counter: {}", *counter.lock().unwrap());
// }

fn test_rc() {
    let rc = Rc::new(0);
    println!("the count is : {}", Rc::strong_count(&rc));
    let rc1 = rc.clone();
    println!("the count is : {}", Rc::strong_count(&rc1));
}

struct Fields {
    regular_field: u8,
    optional_field: Cell<u8>,
}

fn test_struct_cell() {
    // 必需设置成可变的
    let mut fields = Fields {
        regular_field: 3,
        optional_field: Cell::new(2),
    };

    let value = 10u8;
    fields.optional_field = Cell::from(value);

    println!("the cell value is {}", fields.optional_field.get());
}

fn test_ref_cell() {
    let shared_map = Rc::new(RefCell::new(HashMap::new()));
    {
        let mut mut_map = shared_map.borrow_mut();
        mut_map.insert("yan", 2);
        mut_map.insert("yang", 3);
    }
    let value = shared_map.borrow();
    println!("{:?}", value);
}


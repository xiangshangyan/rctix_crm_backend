use std::rc::Rc;
use std::sync::Mutex;
use std::thread;
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

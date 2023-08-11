use tokio::time::Duration;
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let async_task_handle = tokio::spawn(test_async());
    println!("11111111111111");
    // 会休眠两秒钟后继续执行
    async_task_handle.await.unwrap();
    println!("22222222222222")
}

async fn test_async() {
    println!("Async task start...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Async task end...")
}

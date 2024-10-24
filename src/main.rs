use std::time::Duration;

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = sleep(Duration::from_secs(5)) => {
            println!("5초후 타이머 완료");
        },

        _ = my_async_task() => {
            println!("비동기 작업 완료");
        }
    }
}

async fn my_async_task() {
    sleep(Duration::from_secs(3)).await;
    println!("3초 비동기 작업 완료");
}
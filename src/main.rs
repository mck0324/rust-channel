use std::time::Duration;

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("비동기 태스크 시작");
        sleep(Duration::from_secs(2)).await;
        println!("비동기 태스크 완료");
    });
    println!("메인테스크 실행중");
    handle.await.unwrap();
}

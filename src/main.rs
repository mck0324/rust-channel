use std::time::Duration;

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let task1 = async {
        sleep(Duration::from_secs(2)).await;
        println!("Task 1 완료");
    };
    let task2 = async {
        sleep(Duration::from_secs(3)).await;
        println!("Task 2 완료");
    };
    let task3 = async {
        sleep(Duration::from_secs(1)).await;
        println!("Task 3 완료");
    };
    let task4 = async {
        sleep(Duration::from_secs(4)).await;
        println!("Task 4 완료");
    };
    
    tokio::join!(task1,task2,task3,task4);
    println!("모든 태스크 오나료")
}

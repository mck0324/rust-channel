use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send("완료").unwrap();
    });

    let result = rx.await;
    println!("받은 값: {:?}", result);
}

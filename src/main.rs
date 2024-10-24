use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        tx.send("Broad Msg").unwrap();
    });
    tokio::spawn(async move {
        if let Ok(msg) = rx1.recv().await {
            println!("Recv 1: {}",msg);
        }
    });
    if let Ok(msg) = rx2.recv().await {
        println!("Recv 2: {}",msg);
    }
}

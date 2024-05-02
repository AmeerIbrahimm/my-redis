use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx1 = tx.clone();

    let t1 =tokio::spawn(async move {
        let cmd = Command::Get { key: "foo".to_string() };
        tx.send(cmd).await.unwrap();
    });
    let t2 =tokio::spawn(async move {
        let cmd = Command::Set { key: "foo".to_string(), val: "bar".into() };
        tx1.send(cmd).await.unwrap();
    });

    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start reciving messages
        while let Some(command) = rx.recv().await {
            use Command::*;

            match command {
                Get { key } => {
                    client.get(&key).await;
                }
                Set { key, val } => {
                    client.set(&key, val).await;
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}

use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx1 = tx.clone();

    let t1 =tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get { key: "foo".to_string(), resp: resp_tx };
        // Send the GET request
        tx.send(cmd).await.unwrap();
        // await for response
        let res = resp_rx.await;
        println!("GOT: {:?}",res);
    });
    let t2 =tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set { key: "foo".to_string(), val: "bar".into(), resp: resp_tx };
        // Send the SET request
        tx1.send(cmd).await.unwrap();
        // await for response
        let res = resp_rx.await;
        println!("GOT: {:?}",res);
    });

    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start reciving messages
        while let Some(command) = rx.recv().await {
            use Command::*;

            match command {
                Get { key, resp } => {
                    let res =client.get(&key).await;
                    let _ = resp.send(res);
                }
                Set { key, val ,resp} => {
                    let res =client.set(&key, val).await;
                    let _ = resp.send(res);
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
    Get { key: String, resp: Responder<Option<Bytes>> },
    Set { key: String, val: Bytes,resp: Responder<()> },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

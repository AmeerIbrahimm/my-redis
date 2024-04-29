use mini_redis::{ Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Bind the listener to Address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{Get,Set,self};
    use std::collections::HashMap;

    // A hashmap to store data
    let mut db  = HashMap::new();

    // The `Connection` let us read/write redis **frames** instead of byte streams
    // Type is defined by mini-redis
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            ,
            cmd => {
                panic!("unimplemented: {:?}",cmd)
            }

        };
        // write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

use tokio::net::{TcpListener, TcpStream};
use mini_redis::{ Connection, Frame};

#[tokio::main]
async fn main() {
    // Bind the listener to Address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let ( socket,_) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    // The `Connection` let us read/write redis **frames** instead of byte streams
    // Type is defined by mini-redis
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented".into());
        connection.write_frame(&response).await.unwrap();
    }
}

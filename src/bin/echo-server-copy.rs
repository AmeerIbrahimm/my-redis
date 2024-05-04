use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6124").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf =[0;1024];
            // let (mut rd, mut wr) = socket.split();
            loop {
                match socket.read(&mut buf).await {
                    // return if connection end
                    Ok(0) => return ,
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            return ;
                        }
                    },
                    Err(_) => {
                        // we can't do anyting here
                        return;
                    }
                }
            }
        });
    }
}

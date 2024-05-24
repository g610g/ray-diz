use tokio::net::TcpListener;

extern crate redis_client;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    loop {
        let (socket, socket_addr) = listener.accept().await.unwrap();
    }
}

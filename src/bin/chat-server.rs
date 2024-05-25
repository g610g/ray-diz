use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

extern crate redis_client;

async fn read_write_buffer(mut socket: TcpStream) {
    let (read, mut write) = socket.split();
    let mut reader = BufReader::new(read);
    let mut line = String::new();
    loop {
        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 {
            break;
        }
        write.write_all(&line.as_bytes()).await.unwrap();
        line.clear();
    }
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    //binds connection
    let listener = TcpListener::bind("localhost:8000").await.unwrap();
    // let mut handles = vec![];
    //infinitely accepts client and spawn a task for read and writing buffer
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            read_write_buffer(socket).await;
        });
    }
}

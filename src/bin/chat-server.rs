use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
};

extern crate redis_client;

async fn read_write_buffer(mut socket: TcpStream, tx: Sender<String>, mut rx: Receiver<String>) {
    let (read, mut write) = socket.split();
    let mut reader = BufReader::new(read);
    let mut line = String::new();
    loop {
        tokio::select! {
                res = reader.read_line(&mut line) => {
                    let res = res.unwrap();
                    if res == 0 {
                        break;
                }
                let _ = tx.send(line.clone());
        }
                res = rx.recv() => {
                    let msg = res.unwrap();
                let _ = write.write_all(&msg.as_bytes()).await.unwrap();
                }

            }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    //binds connection
    let listener = TcpListener::bind("localhost:8000").await.unwrap();
    let (tx, _) = broadcast::channel::<String>(10);
    //infinitely accepts client and spawn a task for read and writing buffer
    loop {
        let tx_clone = tx.clone();
        let rx_handle = tx.subscribe();
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            read_write_buffer(socket, tx_clone, rx_handle).await;
        });
    }
}

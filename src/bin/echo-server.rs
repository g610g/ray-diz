use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

fn gives_default<T>() -> T
where
    T: Default,
{
    Default::default()
}
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let value: i32 = gives_default();
    let listener = TcpListener::bind("127.0.0.1:6421").await?;
    let (sender, _) = broadcast::channel(10);
    loop {
        let sender = sender.clone();
        let mut reciever = sender.subscribe();
        let (mut socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line_read = String::new();
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line_read) => {
                            if result.unwrap() == 0 {
                            break;
                        }
                            sender.send((line_read.clone(), addr)).unwrap();
                            line_read.clear();
                    }
                    result = reciever.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        println!("{}", other_addr);
                        if addr != other_addr{
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }

                    }

                }
            }
        });
    }
}

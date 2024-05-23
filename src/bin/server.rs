use bytes::Bytes;
use dashmap::DashMap;
use mini_redis::{Connection, Frame};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, Sender};
use tokio::time::sleep;

type ShardedDb = Arc<DashMap<String, Bytes>>;

fn new_sharded_db(shared_num: usize) -> ShardedDb {
    Arc::new(DashMap::with_capacity(shared_num))
}
async fn one_second() {
    sleep(Duration::from_millis(1000)).await;
    println!("one second sleep is done");
}
async fn five_second() {
    sleep(Duration::from_millis(5000)).await;
    println!("five second sleep is done");
}
async fn two_second() {
    sleep(Duration::from_millis(2000)).await;
    println!("two second sleep is done");
}
async fn test_concurrency(tx: Sender<String>) {
    // let mut tasks_send = vec![];
    for _ in 0..32 {
        let tx2 = tx.clone();
        let handle = tokio::spawn(async move {
            two_second().await;
            tx2.send("Two Seconds".to_string()).await;
        })
        .await;
    }
    // for task in tasks_send {
    //     task.await;
    // }
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = new_sharded_db(5);
    let (tx, mut rx) = mpsc::channel(32);
    let t1 = tokio::spawn(async {
        test_concurrency(tx).await;
    });
    while let Some(str) = rx.recv().await {
        println!("{str}");
    }
    //     let (socket, _) = listener.accept().await.unwrap();
    //     let db = db.clone();
    //     println!("Accepted");
    //     tokio::spawn(async move { process(socket, db).await });
    // }
}

async fn process(socket: TcpStream, db: ShardedDb) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("Ok".to_string())
            }
            Get(cmd) => {
                let x = if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                };
                x
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
async fn wait() {
    let mut tasks = vec![];
    for _ in 0..30 {
        let handle = tokio::spawn(async move {
            two_second().await;
        });
        tasks.push(handle);
    }
    for task in tasks {
        let _ = task.await;
    }
}

use bytes::Bytes;
use dashmap::DashMap;
use mini_redis::{Connection, Frame};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep;
type shardedDb = Arc<DashMap<String, Bytes>>;
async fn _foo(_: String) {}
async fn bar() -> String {
    log::info!("Sleeping");
    sleep(Duration::from_secs(4)).await;
    println!("Async bar");
    log::info!("Awake");
    String::from("Hello World")
}
fn new_sharded_db(shared_num: usize) -> shardedDb {
    Arc::new(DashMap::with_capacity(shared_num))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = new_sharded_db(5);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        print!("Accepted");
        tokio::spawn(async move { process(socket, db).await });
    }
}
async fn process(socket: TcpStream, db: shardedDb) {
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
async fn process_db_data(i: i32) {
    read_from_db(i).await;
    println!("processing from process number {i}");
}
async fn read_from_db(i: i32) -> String {
    sleep(Duration::from_millis(1000)).await;
    return "string from".to_string() + &i.to_string();
}

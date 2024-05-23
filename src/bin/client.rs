use bytes::Bytes;
use dashmap::mapref::one;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let mut tasks = vec![];
    for _ in 0..100 {
        let tx2 = tx.clone();
        let handle = tokio::spawn(async move {
            let (resp_tx, resp_rx) = oneshot::channel();
            let cmd = Command::Set {
                key: "bar".to_string(),
                val: "Hello world".into(),
                resp: resp_tx,
            };

            tx2.send(cmd).await.unwrap();
            let result = resp_rx.await.unwrap();
            println!("Got {:?}", result);
        });
        tasks.push(handle);
    }
    // let t1 = tokio::spawn(async move {
    //     let (resp_tx, resp_rx) = oneshot::channel();
    //     let cmd = Command::Get {
    //         key: "foo".to_string(),
    //         resp: resp_tx,
    //     };
    //     tx.send(cmd).await.unwrap();
    //     let res = resp_rx.await;
    //     println!("Got {:?}", res);
    // });
    // let t2 = tokio::spawn(async move {
    //     let (resp_tx, resp_rx) = oneshot::channel();
    //     let cmd = Command::Set {
    //         key: "foo".to_string(),
    //         resp: resp_tx,
    //         val: "bar".into(),
    //     };
    //     tx2.send(cmd).await.unwrap();
    //     let res = resp_rx.await;
    //     println!("Got {:?}", res);
    // });
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    resp.send(res);
                }
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    resp.send(res);
                }
            }
        }
    });
    manager.await.unwrap();
    for task in tasks {
        let _ = task.await;
    }
}

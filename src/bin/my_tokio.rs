use std::{
    collections::VecDeque,
    future::{self, Future},
    pin::Pin,
    task,
};

extern crate redis_client;
struct MyTokio {
    tasks: VecDeque<Task>,
}
type Task = Pin<Box<dyn Future<Output = ()> + Send>>;
impl MyTokio {
    fn new() -> MyTokio {
        MyTokio {
            tasks: VecDeque::new(),
        }
    }
    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future))
    }
}
fn main() {}

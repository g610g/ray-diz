use mini_redis::{client, Result};
async fn foo() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("Hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);
    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    foo().await?;
    Ok(())
}


use apigpio::*;

#[tokio::main]
async fn main() {
  let mut conn = Connection::new().await.expect("connect");
  println!("connected");
}
                                                     

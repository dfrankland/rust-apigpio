
use apigpio::*;

#[tokio::main]
async fn main() {
  let mut conn = BoardConnection::new().await.expect("connect");
  println!("connected");
}
                                                     

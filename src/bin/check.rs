
use apigpio::*;

#[tokio::main]
async fn main() {
  let mut conn = Connection::new().await.expect("connect");
  println!("connected");
  let m = conn.get_mode(14).await.expect("getmode");
  println!("gpio mode {}", m);
}
                                                     

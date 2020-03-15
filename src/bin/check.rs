
use apigpio::*;

use std::time;
use tokio::time::delay_for;

#[tokio::main]
async fn main() {
  let mut args = std::env::args();
  let _ : String = args.next().expect("argv0");
  let pin : Pin = args.next().expect("missing pin").parse().expect("bad pin");
  let conn = Connection::new().await.expect("connect");
  println!("connected");
  let m = conn.get_mode(pin).await.expect("getmode");
  println!("gpio mode {}", m);
  {
    let mut sub = conn.notify_subscribe(pin, true, false)
      .await.expect("subscribe");
    for _i in 0..3 {
      let trans = sub.recv().await.expect("event");
      println!("{:?}", trans);
    }
  }
  delay_for(time::Duration::from_millis(500)).await;
}
                                                     

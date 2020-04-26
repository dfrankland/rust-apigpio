// Copyright 2020 Ian Jackson
// SPDX-License-Identifier: AGPL-3.0-or-later
// There is NO WARRANTY.

use apigpio::*;

#[tokio::main]
async fn main() {
  let mut args = std::env::args();
  let _ : String = args.next().expect("argv0");
  let pin : Pin = args.next().expect("missing pin").parse().expect("bad pin");
  let conn = Connection::new().await.expect("connect");
  println!("connected");
  let m = conn.get_mode(pin).await.expect("getmode");
  println!("gpio mode {}", m);
  let mut sub = conn.notify_subscribe(pin, true, false)
    .await.expect("subscribe");

  loop {
    tokio::select! {
      trans = sub.recv() => println!("{:?}", trans.expect("event")),
    }
  }
}
                                                  

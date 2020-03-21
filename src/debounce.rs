
use tokio::{task,time};

use super::*;

pub struct Debounce {
  output : GpioReceiver,
}

impl Deref for Debounce {
  type Target = GpioReceiver;
  fn deref(&self) -> &Self::Target { &self.output }
}
impl DerefMut for Debounce {
  fn deref_mut(&mut self) -> &mut Self::Target { &mut self.output }
}

impl Debounce {
  pub async fn new_filter(mut input : GpioReceiver,
                          delays : [std::time::Duration ; 2]) -> Self {
    let (forward, output) = watch::channel(*input.borrow());
    task::spawn(async move {
      'await_recv: loop {
        let mut recvd = input.recv().await;

        'just_recvd: loop {
          if recvd.is_none() { break 'await_recv; }; // tearing down?
          let proposed = recvd.unwrap();
          if proposed.level.is_none() { continue 'await_recv; } // startup
          let new_level = proposed.level.unwrap();

          let delay = delays[new_level as usize];
          let timeout = Some(time::delay_for(delay));

          tokio::select! {
            update = input.recv() => {
              recvd = update;
              continue 'just_recvd;
            }
            _ = timeout.unwrap() => {
              let r = forward.broadcast(proposed);
              if r.is_err() { break 'await_recv; }
              continue 'await_recv;
            }
          }
        } // 'just_recvd loop
      } // 'await_recv loop
    });
    Debounce { output }
  }
}

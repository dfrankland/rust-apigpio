
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
      'recv: loop {
        let mut recvd = input.recv().await;

        'pause: loop {
          let proposed = if let Some(p) = recvd { p }
            else { break 'recv; }; // producer went away!  tearing down?
          let new_level = if let Some(l) = proposed.level { l }
            else { continue 'recv; };

          let delay = delays[new_level as usize];
          let timeout = Some(time::delay_for(delay));

          tokio::select! {
            update = input.recv() => {
              recvd = update;
              continue 'pause;
            }
            _ = timeout.unwrap() => {
              let r = forward.broadcast(proposed);
              if r.is_err() { break 'recv; }
              continue 'recv;
            }
          }
        } // 'pause loop
      } // 'recv loop
    });
    Debounce { output }
  }
}

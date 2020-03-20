
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
  pub fn new_filter(input : GpioReceiver, delays : [Tick ; 2]) -> Self {
    let (forward, output) = watch::channel(*input.borrow());
    task::spawn(async move {
      let mut timeout = None;
      loop {
        tokio::select! {
          _ = forward.closed() => {
            break;
          },
          Some(ref update @ GpioChange { level : Some(level), .. })
          = input.recv() => {
            if update.level != forward.borrow().level() {
              let delay = delays[level as usize];
              timeout = Some(time::delay_for(delay));
            }
          },
          _ = timeout.unwrap(), if timeout.is_some() => {
            timeout = None;
            forward.send(*input.borrow);
          },
        };
      }
    });
    Debounce { output }
  }
}

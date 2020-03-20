
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
  pub fn new_filter(mut input : GpioReceiver,
                    delays : [std::time::Duration ; 2]) -> Self {
    let (mut forward, output) = watch::channel(*input.borrow());
    task::spawn(async move {
      let mut deferred : Option<(GpioChange, time::Delay)> = None;
      loop {
        tokio::select! {
          update = input.recv() => {
            if let Some(GpioChange { level : Some(level), .. })
              = update {
              let delay = delays[level as usize];
              deferred = Some((update.unwrap(), time::delay_for(delay)));
            }
          },
          _ = deferred.unwrap().1, if deferred.is_some() => {
            forward.broadcast(deferred.unwrap().0);
            deferred = None;
          },
          _ = forward.closed() => {
            break;
          },
        };
      }
    });
    Debounce { output }
  }
}


use tokio::{task,time};
use tokio::sync::watch;
use tokio::time::Duration;

use super::*;

pub type Debounce = DebounceAny<GpioChange>;

pub struct DebounceAny<S : Debounceable> {
  output : watch::Receiver<S>,
}

impl<S : Debounceable> Deref for DebounceAny<S> {
  type Target = watch::Receiver<S>;
  fn deref(&self) -> &Self::Target { &self.output }
}
impl<S : Debounceable> DerefMut for DebounceAny<S> {
  fn deref_mut(&mut self) -> &mut Self::Target { &mut self.output }
}

pub trait Debounceable : Copy + Send + Sync {
  type Valid : Copy + Send + Sync + 'static;
  fn valid(&self) -> Option<Self::Valid>;
  fn equivalent(a : Self::Valid, b : Self::Valid) -> bool;
}

impl Debounceable for GpioChange {
  type Valid = Level;
  fn valid(&self) -> Option<Self::Valid> { self.level }
  fn equivalent(a : Self::Valid, b : Self::Valid) -> bool { a == b }
}

impl<S : Debounceable> DebounceAny<S> {
  pub async fn new_filter(mut input : watch::Receiver<S>,
                          delays : Box<dyn Send + Fn(S::Valid) -> Duration>)
                          -> Self
  where S : 'static + Debounceable
  {
    let initial = *input.borrow();
    let (forward, output) = watch::channel(initial);
    let mut current = initial.valid();
    task::spawn(async move {
      'await_recv: loop {
        let mut recvd = input.recv().await;

        'just_recvd: loop {
          if recvd.is_none() { break 'await_recv; } // tearing down
          let proposed : S = recvd.unwrap();

          let valid = proposed.valid();
          match (current, valid) {
            (_, None) => continue 'await_recv, // startup
            (Some(cv), Some(nv)) =>
              if <S as Debounceable>::equivalent(cv,nv) {
                continue 'await_recv;
              },
            _ => (),
          };

          let delay = delays(valid.unwrap());
          let timeout = Some(time::delay_for(delay));

          tokio::select! {
            update = input.recv() => {
              recvd = update;
              continue 'just_recvd;
            }
            _ = timeout.unwrap() => {
              current = valid;
              let r = forward.broadcast(proposed);
              if r.is_err() { break 'await_recv; } // receivers gone
              continue 'await_recv;
            }
          }
        } // 'just_recvd loop
      } // 'await_recv loop
    });
    DebounceAny { output }
  }
}

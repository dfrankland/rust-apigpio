
use std::collections::vec_deque::VecDeque;
use std::task::Poll::*;
//use std::io::BufRead;
use std::pin::Pin;
use std::default::Default;

use futures_util::future;

use tokio::io::{AsyncRead,AsyncWrite};

pub struct Communicator<RW : AsyncWrite + AsyncRead> {
  write_buffer  : VecDeque<u8>,
  read_expected : usize,
  connection    : Pin<Box<RW>>,
}

type Result<T> = tokio::io::Result<T>;

trait Outputter {
  type BufObj;
  fn done(&self) -> bool;
  fn got(&mut self, count : usize);
  fn buf<'a>(&'a mut self, bo : &'a mut Self::BufObj) -> &'a mut [u8];
  fn bufobj(&self) -> Self::BufObj;
}

impl Outputter for (usize, &mut [u8]) {
  type BufObj = ();
  fn done(&self) -> bool { self.0 >= self.1.len() }
  fn buf<'a>(&'a mut self, _bo : &'a mut Self::BufObj) -> &'a mut [u8] {
    self.1.split_at_mut(self.0).1
  }
  fn got(&mut self, count : usize) {
    self.0 += count
  }
  fn bufobj(&self) -> Self::BufObj { Default::default() }
}

impl Outputter for usize {
  type BufObj = [u8;20];
  fn done(&self) -> bool { *self == 0 }
  fn buf<'a>(&'a mut self, bo : &'a mut Self::BufObj) -> &'a mut [u8] { bo }
  fn got(&mut self, count : usize) { *self -= count }
  fn bufobj(&self) -> Self::BufObj { Default::default() }
}

impl<RW : AsyncWrite + AsyncWrite + AsyncRead + Unpin> Communicator<RW> {
  pub fn new(connection : RW) -> Communicator<RW> {
    Communicator {
      write_buffer  : VecDeque::new(),
      read_expected : 0,
      connection    : Box::pin(connection),
    }
  }

  pub async fn communicate(&mut self, cmdvec : &[&[u8]], response : &mut [u8])
                           -> Result<()> {
    self.drain_write_buffer().await?;
    self.ignore_stale_responses().await?;
    self.assert_idle();
    for &cmd in cmdvec { self.write_buffer.extend(cmd); }
    self.read_expected = response.len();
    read_into(&mut self.connection, &mut (0, response)).await?;
    self.assert_idle();
    Ok(())
  }

  pub fn into_inner(self) -> RW {
    self.assert_idle();
    *Pin::into_inner(self.connection)
  }

  fn assert_idle(&self) {
    assert!(self.write_buffer.is_empty());
    assert!(self.read_expected == 0);
  }

  async fn drain_write_buffer(&mut self) -> Result<()> {
    loop {
      if self.write_buffer.len() == 0 { break Ok(()) }
      future::poll_fn(|cx|{
        let (slice1, slice2) = self.write_buffer.as_slices();
        let slice = if slice1.is_empty() { slice2 } else { slice1 };
        let got = self.connection.as_mut().poll_write(cx, slice);
        match got {
          Ready(Ok(n)) => {
            self.write_buffer.drain(0..n);
            Ready(<Result<()>>::Ok(()))
          },
          Ready(Err(e)) => Err(e)?,
          Pending => Pending,
        }
      }).await?;
    }
  }

  async fn ignore_stale_responses(&mut self) -> Result<()> {
    read_into(&mut self.connection, &mut self.read_expected).await
  }
}

//impl<RW : AsyncWrite + AsyncRead, O : Outputter> Communicator<RW> {
//  async fn read_into(&mut self, o : &mut O) -> Result<()> {
async fn read_into<RW : AsyncWrite + AsyncRead, O : Outputter>
  (c : &mut Pin<Box<RW>>, o : &mut O) -> Result<()> {
    let mut bo = o.bufobj();
    loop {
      if o.done() { break }
      future::poll_fn(|cx|{
        let got = c.as_mut().poll_read(cx, o.buf(&mut bo));
        match got {
          Ready(Ok(n)) => {
            o.got(n);
            Ready(<Result<()>>::Ok(()))
          },
          Ready(Err(e)) => Err(e)?,
          Pending => Pending,
        }
      }).await?;
    }
    Ok(())
  }

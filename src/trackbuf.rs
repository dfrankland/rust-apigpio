
use std::collections::vec_deque::VecDeque;
use std::task::Poll::*;
use std::io::BufRead;
use std::pin::Pin;

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
  fn bufobj(&self) -> Self::BufObj;
  fn buf(&mut self, bo : &mut Self::BufObj) -> &mut [u8];
}

impl Outputter for &mut [u8] {
  type BufObj = ();
  fn buf(&mut self, bo : &mut Self::BufObj) -> &mut [u8] { *self }
  fn got(&mut self, count : usize) { self.consume(count); }
}

impl Outputter for usize {
  type BufObj = [u8;20];
  fn buf(&mut self, bo : &mut Self::BufObj) -> &mut [u8] { bo }
  fn got(&mut self, count : usize) { *self -= count }
}

impl<RW : AsyncWrite + AsyncWrite + AsyncRead + Unpin> Communicator<RW> {
  pub async fn communicate(&mut self, cmd : &[u8], response : &mut [u8])
                           -> Result<()> {
    self.drain_write_buffer().await?;
    self.ignore_stale_responses().await?;
    self.assert_idle();
    self.write_buffer.extend(cmd);
    self.read_expected = response.len();
    read_into(&mut self, &mut response).await?;
    self.assert_idle();
    Ok(())
  }

  fn assert_idle(&mut self) {
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
    read_into(&mut self, &mut self.read_expected).await
  }
}

//impl<RW : AsyncWrite + AsyncRead, O : Outputter> Communicator<RW> {
//  async fn read_into(&mut self, o : &mut O) -> Result<()> {
async fn read_into<RW : AsyncWrite + AsyncRead, O : Outputter>(c : &mut Communicator<RW>, o : &mut O) -> Result<()> {
    let bo = o.bufobj();
    loop {
      if o.done() { break }
      future::poll_fn(|cx|{
        let got = c.connection.as_mut().poll_read(cx, o.buf(&mut bo));
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


use std::collections::vec_deque::VecDeque;
use std::task::Poll::*;
//use std::io::BufRead;
use std::pin::Pin;
use std::default::Default;
use std::fmt::{self,Display,Formatter};
use std::result;

use thiserror::Error;

use futures_util::future;

use tokio::io::{AsyncRead,AsyncWrite};

pub struct Communicator<RW : AsyncWrite + AsyncRead> {
  write_buffer  : VecDeque<u8>,
  read_expected : usize,
  connection    : Pin<Box<RW>>,
}

type Result<T> = tokio::io::Result<T>;

#[derive(Error,Debug)]
struct PeerDisconnected { }
impl Display for PeerDisconnected {
  fn fmt(&self, fmt: &mut Formatter) -> result::Result<(), fmt::Error> {
    write!(fmt,"PeerDisconnected")
  }
}

trait Outputter {
  type BufObj;
  fn buf<'a>(&'a mut self, expected : usize,
             bo : &'a mut Self::BufObj) -> &'a mut [u8];
  fn bufobj(&self) -> Self::BufObj;
}

impl Outputter for &mut [u8] {
  type BufObj = ();
  fn buf<'a>(&'a mut self, remaining : usize,
             _bo : &'a mut Self::BufObj) -> &'a mut [u8] {
    self.split_at_mut( self.len() - remaining ).1
  }
  fn bufobj(&self) -> Self::BufObj { Default::default() }
}

impl Outputter for () {
  type BufObj = [u8;20];
  fn buf<'a>(&'a mut self, _remaining : usize,
             bo : &'a mut Self::BufObj) -> &'a mut [u8] { bo }
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

  pub async fn communicate(&mut self, cmdvec : &[&[u8]],
                           mut response : &mut [u8])
                           -> Result<()> {
    self.drain_write_buffer().await?;
    self.ignore_stale_responses().await?;
    self.assert_idle();
    for &cmd in cmdvec { self.write_buffer.extend(cmd); }
    self.read_expected = response.len();
    self.drain_write_buffer().await?;
    read_into(&mut self.connection, &mut self.read_expected,
              &mut response).await?;
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
      if self.write_buffer.len() == 0 { break }
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
    Ok(())
  }

  async fn ignore_stale_responses(&mut self) -> Result<()> {
    read_into(&mut self.connection, &mut self.read_expected, &mut ()).await
  }
}

//impl<RW : AsyncWrite + AsyncRead, O : Outputter> Communicator<RW> {
//  async fn read_into(&mut self, o : &mut O) -> Result<()> {
async fn read_into<RW : AsyncWrite + AsyncRead, O : Outputter>
  (c : &mut Pin<Box<RW>>, remaining : &mut usize, o : &mut O) -> Result<()> {
    let mut bo = o.bufobj();
    loop {
      if *remaining == 0 { break }
      future::poll_fn(|cx|{
        let got = c.as_mut().poll_read(cx, o.buf(*remaining, &mut bo));
        match got {
          Ready(Ok(0)) => {
            Ready(Err(
              tokio::io::Error::new(
                tokio::io::ErrorKind::UnexpectedEof, PeerDisconnected{}
              )
            ))
          },
          Ready(Ok(n)) => {
            *remaining -= n;
            Ready(<Result<()>>::Ok(()))
          },
          Ready(Err(e)) => Err(e)?,
          Pending => Pending,
        }
      }).await?;
    }
    Ok(())
  }

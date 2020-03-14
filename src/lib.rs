
mod constants;
use constants::*;

use std::env;

use thiserror::Error;

use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::prelude::*;

use arrayref::*;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use strum_macros::Display;

pub type Word = u32;

pub type MessageBuf = [u8;16];

#[derive(Error,Debug)]
pub enum Error {
  #[error("pigpiod reported error")]
  Pi(i32),
  #[error("env var {0} contains non-unicode data")]
  EnvNotUnicode(String),
  #[error("env var {0} value could not be parsed")]
  EnvInvalidSyntax(String),
  #[error("socket trouble communicating with pigpiod")]
  DaemonComms(#[from] tokio::io::Error),
  #[error("pigpiod unexpectedly sent positive return value {0}")]
  BadReturn(Word),
  #[error("pigpiod sent unexpected gpio mode {0}")]
  BadGpioMode(Word),
  #[error("pigpiod sent reply which did not match our command")]
  ReplyMismatch(Box<(MessageBuf,MessageBuf)>),
}

pub type Result<T> = std::result::Result<T,Error>;

pub struct Connection {
  conn : Mutex<TcpStream>,
}

const PI_ENVPORT : &str = "PIGPIO_PORT";
const PI_ENVADDR : &str = "PIGPIO_ADDR";
const PI_DEFAULT_SOCKET_PORT : u16 = 8888;
const PI_DEFAULT_SOCKET_ADDR : &str = "localhost";

#[derive(Debug,FromPrimitive,Display)]
pub enum GpioMode {
  Input  = PI_INPUT  as isize,
  Output = PI_OUTPUT as isize,
  Alt0   = PI_ALT0 as isize,
  Alt1   = PI_ALT1 as isize,
  Alt2   = PI_ALT2 as isize,
  Alt3   = PI_ALT3 as isize,
  Alt4   = PI_ALT4 as isize,
  Alt5   = PI_ALT5 as isize,
}

#[derive(Debug,Display)]
pub enum Level {
  L = 0,
  H = 1,
}

use Level::*;

impl Level {
  pub fn u(u : usize) -> Level { match u {
    0 => L,
    1 => H,
    _ => panic!("Level::u({})",u)
  } }
  pub fn b(b : bool) -> Level { Level::u(b as usize) }
}

pub type PullUpDown = Option<Level>;

fn env_var(varname : &str) -> Result<Option<String>> {
  use std::env::VarError::*;
  match env::var(varname) {
    Ok(val) => Ok(Some(val)),
    Err(NotPresent) => Ok(None),
    Err(NotUnicode(_)) => Err(Error::EnvNotUnicode(varname.to_owned())),
  }
}

fn default_port() -> Result<u16> {
  let spec = env_var(PI_ENVPORT)?;
  if spec.is_none() { return Ok(PI_DEFAULT_SOCKET_PORT); }
  let spec = spec.unwrap();
  let port : u16 = spec.parse()
    .or_else(|_| Err(Error::EnvInvalidSyntax(spec.to_owned())))?;
  Ok(port)
}

fn default_addr() -> Result<String> {
  let spec = env_var(PI_ENVADDR)?;
  if spec.is_none() { return Ok(PI_DEFAULT_SOCKET_ADDR.to_owned()); }
  Ok(spec.unwrap().to_owned())
}

impl Connection {
  pub async fn new_at<A : tokio::net::ToSocketAddrs>(addr : &A)
                      -> Result<Connection> {
    let conn = TcpStream::connect(addr).await?;
    let conn = Mutex::new(conn);
    Ok(Connection { conn })
  }

  pub async fn new() -> Result<Connection> {
    let addr = default_addr()?;
    let sockaddr = (addr.as_ref(), default_port()?);
    Connection::new_at(&sockaddr).await
  }

  pub async fn cmdr(&self, cmd : Word, p1 : Word, p2 : Word) -> Result<Word> {
    let mut conn = self.conn.lock().await;
    let mut cmsg = [0u8; 16];
    {
      let mut i = 0;
      let mut f = |v| {
        *array_mut_ref![cmsg,i,4] = u32::to_le_bytes(v);
        i += 4;
      };
      f(cmd);
      f(p1);
      f(p2);
    }
    conn.write_all(&cmsg).await?;
    let mut rmsg = [0u8; 16];
    conn.read_exact(&mut rmsg).await?;
    if rmsg[0..12] != cmsg[0..12] {
      return Err(Error::ReplyMismatch(Box::new((cmsg,rmsg))))
    }
    let res = i32::from_le_bytes(*array_ref![rmsg,12,4]);
    if res < 0 { return Err(Error::Pi(res)); }
    Ok(res as Word)
  }

  pub async fn cmd0(&self, cmd : Word, p1 : Word, p2 : Word) -> Result<()> {
    let res = self.cmdr(cmd,p1,p2).await?;
    if res > 0 { return Err(Error::BadReturn(res as Word)) }
    Ok(())
  }
  
  pub async fn set_mode(&self, pin : Word, mode : GpioMode) -> Result<()> {
    self.cmd0(PI_CMD_MODES, pin, mode as Word).await
  }
  pub async fn get_mode(&self, pin : Word) -> Result<GpioMode> {
    let mode = self.cmdr(PI_CMD_MODEG, pin, 0).await?;
    <GpioMode>::from_u32(mode).ok_or_else(|| Error::BadGpioMode(mode))
  }
  pub async fn set_pull_up_down(&self, pin : Word, pud : PullUpDown)
                                -> Result<()> {
    let mode = match pud {
      None    => PI_PUD_OFF,
      Some(L) => PI_PUD_DOWN,
      Some(H) => PI_PUD_UP,
    };
    self.cmd0(PI_CMD_MODES, pin, mode).await
  }
}

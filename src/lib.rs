
#![allow(dead_code)]

use std::env;

use thiserror::Error;

use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::prelude::*;

#[derive(Error,Debug)]
pub enum Error {
  #[error("env var {0} contains non-unicode data")]
  EnvNotUnicode(String),
  #[error("env var {0} value could not be parsed")]
  EnvInvalidSyntax(String),
  #[error("socket trouble communicating with pigpiod")]
  DaemonComms(#[from] tokio::io::Error),
}

pub type Result<T> = std::result::Result<T,Error>;

pub struct Connection {
  conn : Mutex<TcpStream>,
}

const PI_ENVPORT : &str = "PIGPIO_PORT";
const PI_ENVADDR : &str = "PIGPIO_ADDR";
const PI_DEFAULT_SOCKET_PORT : u16 = 8888;
const PI_DEFAULT_SOCKET_ADDR : &str = "localhost";

pub type Word = u32;

#[derive(Debug)]
pub enum GpioMode {
  Input  = 0,
  Output = 1,
  Alt0   = 4,
  Alt1   = 5,
  Alt2   = 6,
  Alt3   = 7,
  Alt4   = 3,
  Alt5   = 2,
}

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

  pub async fn cmd0(&self, cmd : Word, p1 : Word, p2 : Word) -> Result<()> {
    let mut conn = self.conn.lock().await;
    let mut m = [0u8; 16];
    {
      let mut i = 0;
      let mut f = |v| {
        let b = u32::to_le_bytes(v);
        m[i..][0..4].copy_from_slice(&b);
        i += 4;
      };
      f(cmd);
      f(p1);
      f(p2);
    }
    conn.write_all(&m).await?;
    conn.read_exact(&mut m).await?;
    Ok(())
  }
  
  pub async fn set_mode(&self, pin : Word, mode : GpioMode) -> Result<()> {
    const PI_CMD_MODES : Word = 0;
    self.cmd0(PI_CMD_MODES, pin, mode as Word).await
  }
}

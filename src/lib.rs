
use std::env;

use thiserror::Error;

use tokio::net::TcpStream;
use tokio::prelude::*;

#[derive(Error,Debug)]
pub enum Error {
  #[error("env var {0} contains non-unicode data")]
  EnvNotUnicode(String),
  #[error("env var {0} value could not be parsed")]
  EnvInvalidSyntax(String),
  #[error("socket trouble communicating with pigpiod")]
  DaemonComms(tokio::io::Error),
}

type Result<T> = std::result::Result<T,Error>;

pub struct BoardConnection {
  conn : tokio::net::TcpStream,
}

const PI_ENVPORT : &str = "PIGPIO_PORT";
const PI_ENVADDR : &str = "PIGPIO_ADDR";
const PI_DEFAULT_SOCKET_PORT : u16 = 8888;
const PI_DEFAULT_SOCKET_ADDR : &str = "localhost";


fn env_var(varname : &str) -> Result<Option<String>> {
  use std::env::VarError::*;
  match env::var(varname) {
    Ok(val) => Ok(val),
    Err(NotPresent) => Ok(None),
    Err(NotUnicode(_)) => Err(Error::EnvNotUnicode(varname.to_owned())),
  }
}

fn default_port() -> Result<u16> {
  let spec = env_var(PI_ENVPORT)?;
  if let None = spec { return Ok(PI_DEFAULT_SOCKET_PORT); }
  let port : u16 = spec.parse()
    .or_else(|_| Err(Error::ErrInvalidSyntax(spec.to_owned())))?;
  Ok(port)
}

fn default_addr() -> Result<String> {
  let spec = env_var(PI_ENVADDR);
  if let None = spec { return Ok(PI_DEFAULT_SOCKET_ADDR.to_owned()); }
  return spec.to_owned();
}

impl BoardConnection {
  pub async fn new_at(addr : &SocketAddr) -> Result<BoardConnection()> {
    let conn = TcpStream::connect(addr).await;
    Ok(BoardConnection { conn })
  }

  pub async fn new() -> Result<BoardConnection()> {
    let sockaddr = (default_addr()?, default_port()?);
    let conn = TcpStream::connect(&sockaddr).await;
    Ok(BoardConnection { conn })
  }
}

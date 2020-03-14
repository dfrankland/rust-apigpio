
mod constants;
use constants::*;

use std::env;

use thiserror::Error;

use tokio::net::TcpStream;
use tokio::sync::{Mutex,mpsc,oneshot,watch};
use tokio::task;
use tokio::prelude::*;

use std::convert::TryFrom;
use std::ops::Deref;
use std::sync::Arc;

use arrayref::*;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use strum_macros::Display;

use slotmap::{DenseSlotMap};

use std::{result,fmt};

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
  #[error("invalid Level value {0} (should be 0 or 1)")]
  BadLevel(usize),
  #[error("wave_tx_at reports PI_WAVE_NOT_FOUND")]
  WaveNotFound,
  #[error("pigpiod unexpectedly sent positive return value {0}")]
  ProtocolBadReturn(Word),
  #[error("pigpiod sent unexpected gpio mode value {0}")]
  ProtocolBadGpioMode(Word),
  #[error("pigpiod sent unexpected level value {0}")]
  ProtocolBadLevel(Word),
  #[error("pigpiod sent unexpected boolean value {0}")]
  ProtocolBadBoolean(Word),
  #[error("pigpiod sent reply which did not match our command")]
  ProtocolReplyMismatch(Box<(MessageBuf,MessageBuf)>),
}
use Error::*;

pub type Result<T> = std::result::Result<T,Error>;

type Tick = Word;

pub struct Connection {
  conn : Arc<ConnectionCore>,
  notify_add : Mutex<Option<mpsc::Sender<SubscriptionRequest>>>,
}

pub struct ConnectionCore {
  conn : Mutex<TcpStream>,
}

const PI_ENVPORT : &str = "PIGPIO_PORT";
const PI_ENVADDR : &str = "PIGPIO_ADDR";
const PI_DEFAULT_SOCKET_PORT : u16 = 8888;
const PI_DEFAULT_SOCKET_ADDR : &str = "localhost";

#[derive(Debug,FromPrimitive,Display,PartialEq,Eq,Copy,Clone)]
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

#[derive(Debug,Display,PartialEq,Eq,Copy,Clone)]
pub enum Level {
  L = 0,
  H = 1,
}

pub type Pin = Word;

#[derive(Debug,PartialEq,Eq,Copy,Clone)]
pub struct WaveId (pub Word);

impl fmt::Display for WaveId {
  fn fmt(&self, fmt : &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
    write!(fmt, "WaveId{}", self.0)
  }
}

pub struct Pulse {
  pub on_mask:  Word,
  pub off_mask: Word,
  pub us_delay: Word,
}

use Level::*;

impl TryFrom<usize> for Level {
  type Error = Error;
  fn try_from(u : usize) -> Result<Level> {
    Ok(match u {
      0 => L,
      1 => H,
      _ => return Err(BadLevel(u)),
    })
  }
}
macro_rules! level_try_from { { $t:ident } => {
  impl TryFrom<$t> for Level {
    type Error = Error;
    fn try_from(u : $t) -> Result<Level> { Self::try_from(u as usize) }
  }
} }
level_try_from!{u32}
level_try_from!{u16}
level_try_from!{u8}

impl Level {
  pub fn u(u : usize) -> Level {
    TryFrom::try_from(u).unwrap_or_else(|_| panic!("Level::u({})",u))
  }
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
    let conn = ConnectionCore { conn };
    let conn = Arc::new(conn);
    Ok(Connection { conn, notify_add : Mutex::new(None) })
  }

  pub async fn new() -> Result<Connection> {
    let addr = default_addr()?;
    let sockaddr = (addr.as_ref(), default_port()?);
    Connection::new_at(&sockaddr).await
  }
}

impl Deref for Connection {
  type Target = ConnectionCore;
  fn deref(&self) -> &Self::Target { &self.conn }
}

impl ConnectionCore {
  async fn cmd_raw(&self,
                   cmd : Word, p1 : Word, p2 : Word, p3 : Word,
                   extra : &[u8])
                   -> Result<Word> {
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
      f(p3);
    }
    conn.write_all(&cmsg).await?;
    conn.write_all(extra).await?;
    let mut rmsg = [0u8; 16];
    conn.read_exact(&mut rmsg).await?;
    if rmsg[0..12] != cmsg[0..12] {
      return Err(ProtocolReplyMismatch(Box::new((cmsg,rmsg))))
    }
    let res = i32::from_le_bytes(*array_ref![rmsg,12,4]);
    if res < 0 { return Err(Error::Pi(res)); }
    Ok(res as Word)
  }

  pub async fn cmdr(&self, cmd : Word, p1 : Word, p2 : Word) -> Result<Word> {
    self.cmd_raw(cmd,p1,p2,0,&[0;0]).await
  }

  pub async fn cmd0(&self, cmd : Word, p1 : Word, p2 : Word) -> Result<()> {
    let res = self.cmdr(cmd,p1,p2).await?;
    if res > 0 { return Err(ProtocolBadReturn(res as Word)) }
    Ok(())
  }
  
  pub async fn set_mode(&self, pin : Pin, mode : GpioMode) -> Result<()> {
    self.cmd0(PI_CMD_MODES, pin, mode as Word).await
  }
  pub async fn get_mode(&self, pin : Pin) -> Result<GpioMode> {
    let mode = self.cmdr(PI_CMD_MODEG, pin, 0).await?;
    <GpioMode>::from_u32(mode).ok_or_else(|| ProtocolBadGpioMode(mode))
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
  pub async fn gpio_read(&self, pin : Pin) -> Result<Level> {
    let level = self.cmdr(PI_CMD_READ, pin, 0).await?;
    <Level>::try_from(level).map_err(|_| ProtocolBadLevel(level))
  }
  pub async fn gpio_write(&self, pin : Pin, level : Level) -> Result<()> {
    self.cmd0(PI_CMD_WRITE, pin, level as Word).await
  }

  pub async fn wave_clear(&self) -> Result<()> {
    self.cmd0(PI_CMD_WVCLR, 0,0).await
  }
  pub async fn wave_add_new(&self) -> Result<WaveId> {
    Ok(WaveId( self.cmdr(PI_CMD_WVNEW, 0,0).await? ))
  }
  pub async fn wave_create(&self) -> Result<WaveId> {
    // Caller is responsible for not calling wave_* functions for
    // multiple purposes concurrently - ie, for enforcing the
    // concurrency control implied by pigpiod's interface.
    //
    // Getting this wrong is not a memory safety concern (hence the
    // lack of unsafe) but would cause wrong behaviours.
    Ok(WaveId( self.cmdr(PI_CMD_WVCRE, 0,0).await? ))
  }
  pub async unsafe fn wave_delete(&self, wave : WaveId) -> Result<()> {
    // This is safe if no-one in the whole system ever calls
    // wave_send_using_mode with mode *SYNC*.
    self.cmd0(PI_CMD_WVDEL, wave.0, 0).await
  }

  pub async fn wave_send_once(&self, wave: WaveId) -> Result<Word> {
    self.cmdr(PI_CMD_WVTX, wave.0, 0).await
  }
  pub async fn wave_send_repeat(&self, wave: WaveId) -> Result<Word> {
    self.cmdr(PI_CMD_WVTXR, wave.0, 0).await
  }
  pub async fn wave_tx_stop(&self) -> Result<()> {
    self.cmd0(PI_CMD_WVHLT, 0,0).await
  }
  pub async fn wave_tx_at(&self) -> Result<Option<WaveId>> {
    match self.cmdr(PI_CMD_WVTAT, 0,0).await? {
      PI_NO_TX_WAVE     => Ok(None),
      PI_WAVE_NOT_FOUND => Err(WaveNotFound),
      wave              => Ok(Some(WaveId(wave))),
    }
  }
  pub async fn wave_tx_busy(&self) -> Result<bool> {
    match self.cmdr(PI_CMD_WVBSY, 0,0).await? {
      0     => Ok(false),
      1     => Ok(true),
      wrong => Err(ProtocolBadBoolean(wrong)),
    }
  }

  pub async fn wave_add_generic(&self, pulses : &[Pulse]) -> Result<Word> {
    let mut extra = Vec::with_capacity(pulses.len() * 12);
    for ref pulse in pulses {
      extra.extend( &u32::to_le_bytes( pulse.on_mask  ));
      extra.extend( &u32::to_le_bytes( pulse.off_mask ));
      extra.extend( &u32::to_le_bytes( pulse.us_delay ));
    }
    self.cmd_raw(PI_CMD_WVAG, 0,0, extra.len() as Word, &extra).await
  }

  pub async unsafe fn wave_send_using_mode(&self, wave: WaveId, txmode : Word)
                                           -> Result<Word> {
    // Caller must ensure that if txmode is *SYNC* the "bad things"
    // described in the pigpio docs do not happen.
    //
    // If *any* calls to this function use *SYNC*, then wave_delete
    // is potentially unsafe and all calls to it must be checked.
    //
    // Note that because everything is shared amongst all clients of
    // pigpiod, this might involve auditing your process handling etc.
    //
    // Caller must also ensure that txmode is a valid value.
    self.cmdr(PI_CMD_WVTXM, wave.0, txmode).await
  }
}

/* ----- notifications ----- */

#[derive(Debug,PartialEq,Eq,Copy,Clone)]
pub struct GpioChange {
  pin : Pin,
  level : Option<Level>,
  tick : Option<Tick>, // None for the initial notification
  sequence : Word, // increments by 1 each time; allows spotting lost events
}

pub struct Subscription {
  wreceiver : watch::Receiver<GpioChange>,
  _dsender : oneshot::Sender<()>, // exists to signal being dropped
}
impl Deref for Subscription {
  type Target = watch::Receiver<GpioChange>;
  fn deref(&self) -> &Self::Target { &self.wreceiver }
}

pub struct SubscriptionRecord {
  client : watch::Sender<GpioChange>,
  pin : Pin,
  previously : Option<Word>, // None means sender not ever notified yet
  sequence : Word,
}

slotmap::new_key_type! { struct SubscriptionKey; }

type SubscriptionRequest = (
  Pin,
  watch::Sender<GpioChange>,
  oneshot::Receiver<()>,
  Option<Option<Level>>
);

struct NotificationProcessor {
  conn : Arc<ConnectionCore>,
  add : mpsc::Receiver<SubscriptionRequest>,
  remove_sender : mpsc::Sender<SubscriptionKey>,
  remove_receiver : mpsc::Receiver<SubscriptionKey>,
  subs : DenseSlotMap<SubscriptionKey, SubscriptionRecord>,
}

impl SubscriptionRecord {
  fn notify(&mut self, level : Level, tick : Option<Tick>) {
    // trickily, this means sequence on our first actual notification is
    // 1, whereas the thing we put into the watch when we create it is 0
    self.sequence += 1;
    self.client.broadcast(GpioChange {
        pin : self.pin,
        level : Some(level), sequence : self.sequence,
        tick
    }).ok();
    self.previously = Some((level as Word) << self.pin);
  }
}

impl NotificationProcessor {
  async fn spawn(conn : &Arc<ConnectionCore>)
                 -> Result<mpsc::Sender<SubscriptionRequest>> {
    let conn = conn.clone();
    let (add_sender, add_receiver) = mpsc::channel(5);
    let (remove_sender, remove_receiver) = mpsc::channel(20);
    let subs = DenseSlotMap::with_key();
    // xxx need socket for notifications!
    // xxx need to request notifications inband
    let mut processor = NotificationProcessor {
      add : add_receiver,
      conn, remove_sender, remove_receiver, subs
    };
    task::spawn(async move {
      processor.task().await.expect("gpio change notification task died");
    });
    Ok(add_sender)
  }

  async fn do_add(&mut self, req : SubscriptionRequest) -> Result<()> {
    let (pin, wsender, dreceiver, expected_spec) = req;
    let mut record = SubscriptionRecord {
      client : wsender,
      pin,
      previously : None,
      sequence : 0,
    };
    if let Some(expected) = expected_spec {
      let actual = self.conn.gpio_read(pin).await?;
      if expected != Some(actual) { record.notify(actual, None) }
    };
    let subk = self.subs.insert(record);
    let mut tremove = self.remove_sender.clone();
    task::spawn(async move {
      dreceiver.await.unwrap_err();
      tremove.send(subk).await.ok();
    });
    Ok(())
  }

  async fn task(&mut self) -> Result<()> {
    loop {
      tokio::select! {
        adding = self.add.recv() => match adding {
          Some(req) => self.do_add(req).await?,
          None => return Ok(()),
        },
/* xxx
        () = thing.read => {
          let (seqno, tick, flags, levels) = thing;
          for ref mut record in self.subs {
            let mask = 1u32 << record.pin;
            let thisbit = levels & mask;
            if record.previously != Some(thisbit) {
              record.sequence += 1;
              let level = Level::b(thisbit != 0);
              record.client.broadcast(
                GpioChange { pin, level, tick, sequence : record.sequence }
              ).ok();
              record.previously = Some(thisbit);
            }
          }
        }*/
        unsubk = self.remove_receiver.recv() => {
          let _unsub = self.subs.remove(unsubk.unwrap()).unwrap();
          // drop unusb
        },
      }
    }
  }
}

impl Connection {                   
  pub async fn notify_subscribe(&self, pin : Pin,
                                expected_spec : Option<Option<Level>>)
                                -> Result<Subscription> {
    // Arranges to send GpioChange to Subscription
    // If expected_spec is not None, reads the gpio once at the start,
    // and sends a notification with no Tick if expected_spec is not
    // Some(Some(the current level)).

    let mut notify_add = self.notify_add.lock().await;
    if notify_add.is_none() {
      *notify_add = Some(NotificationProcessor::spawn(&self.conn).await?);
    }

    // xxx do the gpio read here ?

    let (wsender, wreceiver) = watch::channel(GpioChange {
      pin, level : None, tick : None, sequence : 0,
    });
    let (dsender, dreceiver) = oneshot::channel();
    notify_add.as_mut().unwrap()
      .send(( pin, wsender, dreceiver, expected_spec ))
      .await.expect("notify task died?");

    Ok(Subscription { wreceiver, _dsender : dsender })
  }
}

// Copyright 2020 Ian Jackson
// SPDX-License-Identifier: AGPL-3.0-or-later
// There is NO WARRANTY.

/// Unfortunately, this library is not yet properly documented.

pub mod constants;
use constants::*;

pub mod trackbuf;

use std::env;

use thiserror::Error;

use tokio::net::TcpStream;
use std::net::SocketAddr;
use tokio::sync::{Mutex,MutexGuard,mpsc,oneshot,watch};
use tokio::task;
use tokio::prelude::*;

use std::convert::TryFrom;
use std::ops::{Deref,DerefMut};
use std::sync::Arc;
use std::mem::replace;

use arrayref::*;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use strum_macros::Display;

use slotmap::{DenseSlotMap};

use std::{result,fmt};

use trackbuf::Communicator;

pub type Word = u32;

pub type MessageBuf = [u8;16];

#[derive(Error,Debug)]
pub enum Error {
  #[error("pigpiod reported error {0}")]
  Pi(i32),
  #[error("env var {0} contains non-unicode data")]
  EnvNotUnicode(String),
  #[error("env var {0} value could not be parsed")]
  EnvInvalidSyntax(String),
  #[error("failed to get address of pigpiod!")]
  DaemonConnectionNoAddress,
  #[error("failed to connect to pigpiod: {0:?}")]
  DaemonConnectionFailed(Vec<(SocketAddr, tokio::io::Error)>),
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

type Tick = Word; // [us]

#[derive(Clone)]
pub struct Connection {
  conn : Arc<ConnectionCore>,
  addrs : Vec<SocketAddr>,
  _notify_shutdown_sender : mpsc::Sender<()>,
}

pub struct ConnectionCore {
  conn : Mutex<Communicator<TcpStream>>,
  notify : Mutex<NotifyInCore>,
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

#[derive(Debug,Display,PartialEq,Eq,Copy,Clone,Ord,PartialOrd,Hash)]
pub enum Level {
  L = 0,
  H = 1,
}

impl std::ops::Not for Level {
  type Output = Level;
  fn not(self) -> Self { match self { H => L, L => H, } }
}

pub type Pin = Word;

pub struct WaveId (pub Word);

const TICK_KEEPALIVE_US : Word = 60000000;

impl fmt::Display for WaveId {
  fn fmt(&self, fmt : &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
    write!(fmt, "WaveId{}", self.0)
  }
}

#[derive(Clone,Copy,Debug)]
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

async fn connect_from_addrs(addrs : &Vec<SocketAddr>) -> Result<TcpStream> {
  if addrs.len() == 0 { return Err(DaemonConnectionNoAddress) }
  let mut errors = Vec::with_capacity(addrs.len());
  for addr in addrs {
    match TcpStream::connect(addr).await {
      Ok(conn) => return Ok(conn),
      Err(e) => errors.push((addr.clone(), e)),
    }
  }
  Err(DaemonConnectionFailed(errors))
}  

impl Connection {
  pub async fn new_at<A : tokio::net::ToSocketAddrs>(addr : &A)
                      -> Result<Connection> {
    let addrs = tokio::net::lookup_host(addr).await?.collect();
    Connection::new_from_addrs(addrs).await
  }

  async fn new_from_addrs(addrs : Vec<SocketAddr>) -> Result<Connection> {
    let conn = connect_from_addrs(&addrs).await?;
    let (shutdown_sender, for_shutdown) = mpsc::channel(1);
    let notify = Mutex::new(NotifyInCore::Idle { for_shutdown });
    let comm = Communicator::new(conn);
    let core = Arc::new(ConnectionCore { conn : Mutex::new(comm), notify });
    Ok(Connection {
      conn : core, addrs,
      _notify_shutdown_sender : shutdown_sender,
    })
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

async fn cmd_raw(conn : &mut Communicator<TcpStream>,
                 cmd : Word, p1 : Word, p2 : Word, p3 : Word,
                 extra : &[u8])
                 -> Result<Word> {
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
  let mut rmsg = [0u8; 16];
  conn.communicate(&[&cmsg,extra], &mut rmsg).await?;
  if rmsg[0..12] != cmsg[0..12] {
    return Err(ProtocolReplyMismatch(Box::new((cmsg,rmsg))))
  }
  let res = i32::from_le_bytes(*array_ref![rmsg,12,4]);
  if res < 0 { return Err(Error::Pi(res)); }
  Ok(res as Word)
}

impl ConnectionCore {
  async fn cmd_raw(&self,
                   cmd : Word, p1 : Word, p2 : Word, p3 : Word,
                   extra : &[u8])
                   -> Result<Word> {
    let mut conn = self.conn.lock().await;
    cmd_raw(&mut conn, cmd,p1,p2,p3, extra).await
  }

  async fn cmdr(&self, cmd : Word, p1 : Word, p2 : Word) -> Result<Word> {
    self.cmd_raw(cmd,p1,p2,0,&[0;0]).await
  }

  async fn cmd0(&self, cmd : Word, p1 : Word, p2 : Word) -> Result<()> {
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
    self.cmd0(PI_CMD_PUD, pin, mode).await
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
  pub async fn wave_get_micros(&self) -> Result<Word> {
    self.cmdr(PI_CMD_WVSM, 0, 0).await
  }
  pub async fn wave_get_high_micros(&self) -> Result<Word> {
    self.cmdr(PI_CMD_WVSM, 1, 0).await
  }
  pub async fn wave_get_max_micros(&self) -> Result<Word> {
    self.cmdr(PI_CMD_WVSM, 2, 0).await
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

#[derive(Debug,PartialEq,Eq,Copy,Clone,Hash)]
pub struct GpioChange {
  pub pin : Pin,
  pub level : Option<Level>, // None only before gpio has been read
  pub tick : Option<Tick>, // None for the initial notification
  pub sequence : Word, // increments by 1 each time; allows spotting lost events
}

type GpioReceiver = watch::Receiver<GpioChange>;

pub struct Subscription {
  wreceiver : GpioReceiver,
  _dsender : oneshot::Sender<()>, // exists to signal being dropped
}
impl Deref for Subscription {
  type Target = GpioReceiver;
  fn deref(&self) -> &Self::Target { &self.wreceiver }
}
impl DerefMut for Subscription {
  fn deref_mut(&mut self) -> &mut Self::Target { &mut self.wreceiver }
}

#[derive(Debug)]
struct SubscriptionRecord {
  pin : Pin,
  client : watch::Sender<GpioChange>,
  previously : Option<Word>, // None means sender not ever notified yet
  sequence : Word,
  tick_last_keepalive : Option<Option<Word>>,
}

slotmap::new_key_type! { struct SubscriptionKey; }

type NotifyMap = DenseSlotMap<SubscriptionKey, SubscriptionRecord>;

struct NotifyShared {
  nhandle : Word,
  subs : NotifyMap,
  remove_sender : mpsc::Sender<SubscriptionKey>,
}

enum NotifyInCore {
  Active(NotifyShared),
  Idle { for_shutdown : mpsc::Receiver<()> },
}

impl NotifyInCore {
  fn is_idle(&self) -> bool { match self {
    NotifyInCore::Active(_) => false,
    NotifyInCore::Idle{..} => true,
  } }
}

struct NotifyInTask {
  conn : Arc<ConnectionCore>,
  stream : TcpStream,
  shutdown_receiver : mpsc::Receiver<()>,
  remove_receiver : mpsc::Receiver<SubscriptionKey>,
}

struct NotifySubs<'a> {
  guard : MutexGuard< 'a, NotifyInCore>,
}

impl<'a> Deref for NotifySubs<'a> {
  type Target = NotifyShared;
  fn deref(&self) -> &NotifyShared {
    match self.guard.deref() {
      NotifyInCore::Active(ref r) => r,
      _ => panic!(),
    }
  }
}
impl<'a> DerefMut for NotifySubs<'a> {
  fn deref_mut(&mut self) -> &mut NotifyShared {
    match self.guard.deref_mut() {
      NotifyInCore::Active(ref mut r) => r,
      _ => panic!(),
    }
  }
}

impl NotifyInTask {
  async fn task(&mut self) -> Result<()> {
    let mut reportbuf = [0u8; 12];
    loop {
      tokio::select! {
        _ = self.shutdown_receiver.recv() => {
          return Ok(());
        },
        r = self.stream.read_exact(&mut reportbuf) => {
          r?;
          let tick   = u32::from_le_bytes(*array_ref![reportbuf,4,4]);
          let levels = u32::from_le_bytes(*array_ref![reportbuf,8,4]);
          let mut shared = self.conn.lock_notify_shared().await;
          for (_, mut sub) in &mut shared.subs {
            let mask = 1 << sub.pin;
            let now = levels & mask;
            let (do_keepalive, new_last_keepalive)
              = match sub.tick_last_keepalive {
                None =>       (false, None),
                Some(None) => (false, Some(Some(tick))),
                Some(Some(then)) => (tick - then > TICK_KEEPALIVE_US,
                                      Some(Some(tick))),
              };
            if sub.previously != Some(now) || do_keepalive {
              // trickily, this means sequence on our first actual
              // notification is 1, whereas the thing we put into the
              // watch when we create it is 0
              sub.sequence += 1;
              sub.previously = Some(now);
              sub.tick_last_keepalive = new_last_keepalive;
              sub.client.broadcast(GpioChange {
                pin : sub.pin,
                level : Some(Level::b(now != 0)),
                tick : Some(tick),
                sequence : sub.sequence,
              }).ok();
            }
          }
        },
        unsubk = self.remove_receiver.recv() => {
          let mut shared = self.conn.lock_notify_shared().await;
          shared.subs.remove(unsubk.unwrap()).unwrap();
          let conn = self.conn.clone();
          task::spawn(async move {
            task::yield_now().await;
            let mut shared = conn.lock_notify_shared().await;
            conn.notify_tell_pigpiod_locked(&mut shared).await
              .unwrap_or(() /* failed to deregister, ah well */);
          });
        },
      }
    }
  }
}

impl ConnectionCore {
  async fn lock_notify_shared<'b, 'a : 'b>(&'a self) -> NotifySubs<'b> {
    let guard = self.notify.lock().await;
    if guard.is_idle() { panic!(); }
    NotifySubs { guard }
  }

  async fn notify_tell_pigpiod_locked(&self, shared : &mut NotifyShared)
                                      -> Result<()> {
    let mut mask = 0;
    for (_, sub) in &shared.subs {
      mask |= 1 << sub.pin;
    }
    self.cmd0(PI_CMD_NB, shared.nhandle, mask).await
  }
}

impl Connection {
  pub async fn notify_subscribe(&self, pin : Pin,
                                read_initially : bool,
                                tick_keepalives : bool)
                                -> Result<Subscription> {
    // Arranges to send GpioChange to Subscription
    // If expected_spec is not None, reads the gpio once at the start,
    // and sends a notification with no Tick if expected_spec is not
    // Some(Some(the current level)).

    let mut notify_locked = self.notify.lock().await;
    if notify_locked.is_idle() {
      let stream = connect_from_addrs(&self.addrs).await?;
      let mut comm = Communicator::new(stream);
      let nhandle = cmd_raw(&mut comm, PI_CMD_NOIB,0,0,0, &[0;0]).await?;
      let stream = comm.into_inner();

      let subs = DenseSlotMap::with_key();
      let (remove_sender, remove_receiver) = mpsc::channel(20);
      let active = NotifyInCore::Active(
        NotifyShared { nhandle, subs, remove_sender }
      );
      let was = replace(notify_locked.deref_mut(), active);
      let shutdown_receiver = match was {
        NotifyInCore::Idle { for_shutdown : r } => r,
        _ => panic!(),
      };
      let conn = self.conn.clone();

      let mut processor =
        NotifyInTask { conn, stream, shutdown_receiver, remove_receiver, };
      task::spawn(async move {
        processor.task().await.expect("gpio change notification task died");
      });
    }
    let shared = match *notify_locked {
      NotifyInCore::Active(ref mut r) => r,
      _ => panic!(),
    };

    let initial = GpioChange {
      pin, level : None, tick : None, sequence : 0,
    };
    let (wsender, wreceiver) = watch::channel(initial);
    let (dsender, dreceiver) = oneshot::channel();

    let record = SubscriptionRecord {
      pin,
      client : wsender,
      previously : None,
      sequence : 0,
      tick_last_keepalive : if tick_keepalives { Some(None) } else { None },
    };

    let subk = shared.subs.insert(record);
    self.notify_tell_pigpiod_locked(shared).await?;

    if read_initially {
      let level = self.conn.gpio_read(pin).await?;
      let record = &mut shared.subs[subk];
      record.previously = Some((level as Word) << pin);
      record.client.broadcast(GpioChange { level : Some(level), ..initial })
        .ok();
    }

    let mut tremove = shared.remove_sender.clone();
    task::spawn(async move {
      dreceiver.await.unwrap_err();
      tremove.send(subk).await.ok();
    });

    Ok(Subscription { wreceiver, _dsender : dsender })
  }
}

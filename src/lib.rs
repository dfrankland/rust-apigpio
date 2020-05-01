// Copyright 2020 Ian Jackson
// SPDX-License-Identifier: AGPL-3.0-or-later
// There is NO WARRANTY.

//! # Raspberry PI GPIO access library based on pigpiod.
//!
//! This library a pure-Rust analogue to the C `pigpio_if2` library,
//! providing an async interface to talk to pigpiod and based on
//! Tokio.
//!
//! Currently we provide only a subset of pigpio's functionality:
//! raw GPIO access and notifications, and waveform generation.
//! Contributions are of course welcome.
//!
//! # Consider `rppal` instead!
//!
//! You should consider whether you wish to use this library, or
//! instead use the excellent `rppal` library.
//!
//! Advantages of `apigpo`:
//!
//!  * Access to pigpiod's DMA-based wavefile generation.
//!
//!  * Interface is very similar to pigpio's own interface: this could
//!    ease porting existing rpi code to Rust, and may make it easier
//!    to make use of existing online information based on C or Python
//!    pigpio access.
//!
//!  * You can port-forward your pigpiod to a faster or more
//!    convenient computer, and run your program there (during
//!    development, for example).
//!
//!  * Correct, race-free, operation when multiple processes are
//!    changing *modes* of GPIO pins concurrently, provided everything
//!    is using pigpiod.  (Concurrent multiprocess changes to GPIO
//!    *levels* are fine with any library, including `rppal`.)
//!
//! Advantages of `rppal`:
//!
//!  * I2C, PWM, SPI, UART support.  RPI model information, etc.
//!
//!  * Much more Rust-ish interface, with better type safety etc.
//!    For example, you can't forget to set the gpio pin mode.
//!
//!  * Less overhead because GPIO access is done directly within
//!    your Rust program without any syscalls, rather than by
//!    talking to a separate daemon.
//!
//!  * GPIO change notification based on interrupts rather than
//!    pigpiod's polling.
//!
//!  * No need to deal with async Rust.
//!
//!  * No need to arrange for a daemon to be running, make sure
//!    your embedded OS's startup order is correct, etc.
//!
//! It is possible to use both libraries in a single project,
//! but see the following note:
//!
//! # Concurrent setting of RPI GPIO PIN modes
//!
//! The Broadcom SOC provides a way to raise, or lower, individual
//! GPIO pin outputs (or sets of outputs) in a way that does not
//! interfere with other similar operations performed concurrently.
//!
//! However, this interface is only provided for setting the *level*
//! of an output pin.  For modes, pullup/pulldown, etc., there are
//! only registers containing information about multiple pins where
//! changes are made by reading the register, adjusting the bits which
//! control a particular pin, and writing the information back.
//!
//! If multiple tasks on the rpi do this at once, they can
//! accidentally undo each others' changes.
//!
//! For this purpose, pigpiod is a single task: it will serialise the
//! updates itself.  So if all your programs use pigpiod (via apigpio,
//! or via another programming language which talks to pigpiod) you
//! are fine.
//!
//! If you want to mix and match, the easiest way to ensure
//! correctness is to have a single task at startup set all the gpio
//! modes as you want them.  Then all subsequent updates will be
//! harmless no-ops.
//!
//! Another easy way to avoid this problem is to have only a single
//! process using, say, `rppal`.
//!
//! # Safety and correctness
//!
//! apigpio is entirely in safe Rust.  So you should not experience
//! memory corruption within your Rust program.
//!
//! However, there can be some surprises because of the way pigpiod
//! itself works.
//!
//! Firstly: *pigpiod resources are global*.  There is no isolation
//! between different programs all speaking to pigpiod and there is no
//! automatic cleanup.  For example, in particular:
//! 
//! Only one program can conveniently make use of `wave_*` functions
//! at once, because pigpiod has only one currently-building waveform,
//! and one currently-transmitting waveform.  Waveforms are not
//! deleted when your program quits - but it is conventional to use
//! `wave_clear` at startup so your next run will clean everything up.
//!
//! Secondly: *pigpiod itself has some hazardous features*.  These are
//! generally discussed in the pigpio documentation.  The only such
//! feature currently available via apigpio is the `*SYNC*` waveform
//! chaining function combined with the ability to delete waveforms.
//! These kind of features are available in apigpiod only from
//! `unsafe` callers (even though they are implemented in Safe Rust).


pub mod constants;
use constants::*;

pub mod trackbuf;

#[path="constants/errors.rs"]
pub mod errors;

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
use std::fmt::{Display,Formatter};

use arrayref::*;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use strum_macros::Display;

use slotmap::{DenseSlotMap};

use std::{result,fmt};

use trackbuf::Communicator;

/// pigpiod likes to think about most things as 32-bit words.
pub type Word = u32;

/// Used only for reporting an unexpected reply from pigpiod.
pub type MessageBuf = [u8;16];

/// Wraps up a pigpiod error code
#[derive(Debug,Copy,Clone,Eq,PartialEq,Hash)]
pub struct PigpiodError (pub i32);

#[derive(Error,Debug)]
pub enum Error {
  /// This is not an enum because if pigpiod is newer than apigpio,
  /// pigpiod might send error codes that apigpio does not understand.
  /// Specific error values can be checked by comparing with
  /// values from the `constants` module.
  #[error("pigpiod reported error {0}")]
  Pi(PigpiodError),
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

impl Display for PigpiodError {
  fn fmt(&self, f : &mut Formatter) -> result::Result<(), fmt::Error> {
    if let Some((abbrev, desc)) = PI_error_code_lookup(self.0) {
      write!(f, "{} ({})", abbrev, desc)
    } else {
      write!(f, "PI_ unknown error code {}", self.0)
    }
  }
}

pub type Result<T> = std::result::Result<T,Error>;

/// pigpiod tick (\[us])
pub type Tick = Word;

/// The main struct which owns a connection to pigpiod.
/// Start by creating one of these.
///
/// Most of the interesting methods are methods on `ConnectionCore`
/// which this derefs to.
#[derive(Clone)]
pub struct Connection {
  conn : Arc<ConnectionCore>,
  addrs : Vec<SocketAddr>,
  _notify_shutdown_sender : mpsc::Sender<()>,
}

/// Most of the Connection methods, mirroring `pigpiod_if2`,
/// are actually provided here.
///
/// Where methods are named after a facility in `pigpiod_if2`
/// <http://abyz.me.uk/rpi/pigpio/pdif2.html> they perform the same
/// function here.  Documentation here in `agpipio` is present only
/// if there is something unusual.
pub struct ConnectionCore {
  conn : Mutex<Communicator<TcpStream>>,
  notify : Mutex<NotifyInCore>,
}

/// Env var name to override pigpiod port to connect to.
pub const PI_ENVPORT : &str = "PIGPIO_PORT";
/// Env var name to override pigpiod address to connect to.
pub const PI_ENVADDR : &str = "PIGPIO_ADDR";
/// Default pigpiod port.
pub const PI_DEFAULT_SOCKET_PORT : u16 = 8888;
/// Default pigpiod address.
pub const PI_DEFAULT_SOCKET_ADDR : &str = "localhost";

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

/// BCM GPIO pin number.
///
/// For documentation purposes, where a word is a GPIP pin number
/// we use this type.
pub type Pin = Word;

/// Refers to a Wave stored in pigpiod.
///
/// See the pigpio wave documentaton.  In pigpiod Wave IDs are global
/// across all pigpio clients, and waves not cleared (nor transmission
/// stopped!) when a client disconnects.
///
/// Because of safety concerns (see particularly the pigpio documentation
/// about wave `*SYNC*`), waves are not automatically deleted when
/// a `WaveID` is dropped.  The best approach is usually to use
/// `wave_clear` on program startup.
///
/// The inner `Word` is available in case you need to do something
/// exciting (maybe something cross-process).
#[derive(Debug,PartialEq,Eq,Copy,Clone,Ord,PartialOrd,Hash)]
pub struct WaveId (pub Word);

/// Keepalive interval for `notify_subscribe` `tick_keepalives`
pub const TICK_KEEPALIVE_US : Word = 60000000;

impl fmt::Display for WaveId {
  fn fmt(&self, fmt : &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
    write!(fmt, "WaveId{}", self.0)
  }
}

/// Represents a waveform element.  See pigpio docs.
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
  /// Short convenient name for converting into a `Level`.
  pub fn u(u : usize) -> Level {
    TryFrom::try_from(u).unwrap_or_else(|_| panic!("Level::u({})",u))
  }
  /// Short convenient name for converting from a `Level`.
  pub fn b(b : bool) -> Level { Level::u(b as usize) }
}

/// Specifies whether there should be a weak pullup or pulldown.
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
  /// Connects to pigpiod, given specific socket address details.
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

  /// Connects to pigpiod using the default address and port.
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
  if res < 0 { return Err(Error::Pi(PigpiodError(res))); }
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
  /// Caller is responsible for not calling wave_* functions for
  /// multiple purposes concurrently - ie, for enforcing the
  /// concurrency control implied by pigpiod's interface.
  ///
  /// Getting this wrong is not a memory safety concern (hence the
  /// lack of unsafe) but would cause wrong behaviours.
  ///
  /// Note that this applies even across multiple different pigpiod
  /// clients.
  pub async fn wave_create(&self) -> Result<WaveId> {
    Ok(WaveId( self.cmdr(PI_CMD_WVCRE, 0,0).await? ))
  }
  /// This is safe if no-one in the whole system ever calls
  /// `wave_send_using_mode` with mode `*SYNC*`.  See the pigpio
  /// documentation on `wave_send_using_mode` for full details.
  pub async unsafe fn wave_delete(&self, wave : WaveId) -> Result<()> {
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

  /// Caller must ensure that if txmode is `*SYNC*` the "bad things"
  /// described in the pigpio docs do not happen.
  ///
  /// If *any* calls to this function use `*SYNC*`, then `wave_delete`
  /// is potentially unsafe and all calls to it must be checked.
  ///
  /// Note that because everything is shared amongst all clients of
  /// pigpiod, this might involve auditing your process handling etc.
  ///
  /// Caller must also ensure that `txmode` is a valid value.  If it is
  /// not then possibly it invokes some hazardous new feature of
  /// pigpiod. 
  pub async unsafe fn wave_send_using_mode(&self, wave: WaveId, txmode : Word)
                                           -> Result<Word> {
    self.cmdr(PI_CMD_WVTXM, wave.0, txmode).await
  }
}

/* ----- notifications ----- */

/// Represents a change to a gpio pin, as requested by
/// `notify_subscribe`.
#[derive(Debug,PartialEq,Eq,Copy,Clone,Hash)]
pub struct GpioChange {
  pub pin : Pin,
  /// Can be `None` only if `notify_subscribe` parameter
  /// `read_initially` is `false`: then it is `None` before gpio has
  /// been read for the first time.  In that case, the first recv on
  /// the `Subscription` will get `None`.  All other recvs will get
  /// `Some`.
  pub level : Option<Level>,
  /// `None` until the first change.  Ie, the first recv on
  /// `Subscription` will get `None` (regardless of `read_initially`)
  /// and subsequent ones will get `Some`.
  pub tick : Option<Tick>,
  /// Increments by 1 each time the `Subscription` is updated This
  /// allows the caller to spot missed updates, provided that they
  /// look often enough that it doesn't wrap.
  pub sequence : Word,
}

type GpioReceiver = watch::Receiver<GpioChange>;

/// Subscription to a GPIO pin.
///
/// Contains a `GpioReceiver` ie
/// a `watch::Receiver<GpioChange>`.  To unsubscribe from
/// notifications, simply drop the `Subscription`.
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
  /// Starts watching for changes to a GPIO pin.  Change notifications
  /// are sent as `GpioChange` to the returned `Subscription`.
  ///
  /// If `read_initially`, reads the gpio once at the start, so
  /// that the subscription's `level` starts out as `Some`.
  ///
  /// If `tick_keepalives`, will send a `GpioChange` with unchanged
  /// `Some(Level)` and a fresh `Some(Tick)` at least every
  /// `TICK_KEEPALIVE_US`; this allows the receiver to spot when
  /// the tick wraps around.
  ///
  /// Behind the scenes, getting GPIO change notifications involves
  /// making a 2nd connection to pigpiod.  This will be done the first
  /// time `notify_subscribe` is called; it will then be retained for
  /// future reuse.  This is relevant to Rust callers because pigpiod
  /// has a limit on the number of simultaneous connections.
  pub async fn notify_subscribe(&self, pin : Pin,
                                read_initially : bool,
                                tick_keepalives : bool)
                                -> Result<Subscription> {
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

extern crate libc;
extern crate bitflags;

pub extern crate thiserror;

pub mod bindings;

#[macro_use]
pub mod macros;

pub mod error;
pub use self::error::Error;

pub mod r#loop;
pub use self::r#loop::{
  Loop, RunMode, LoopOption
};

pub mod buf;
pub use self::buf::Buf;

pub mod handle;
pub use self::handle::{
  Handle, HandleType, HandleImpl, CloseCb
};

pub mod req;
pub use self::req::{
  Req, ReqType, ReqImpl,
  Write, Connect, Shutdown, 
  WriteCb, ConnectCb, ShutdownCb
};

pub mod stream;
pub use self::stream::{
  Stream, StreamImpl, AllocCb, ReadCb, ConnectionCb
};

pub mod poll;
pub use self::poll::{Poll, PollEvent, PollCb};

pub mod stdio;
pub use self::stdio::{StdioFlags, StdioContainer};

pub mod process;
pub use self::process::{Process, ProcessOptions};

pub mod signal;
pub use self::signal::{Signal, SignalCb};

pub mod idle;
pub use self::idle::{Idle, IdleCb};

pub mod check;
pub use self::check::{Check, CheckCb};

pub mod prepare;
pub use self::prepare::{Prepare, PrepareCb};

pub mod timer;
pub use self::timer::Timer;

pub mod pipe;
pub use self::pipe::Pipe;

pub mod tcp;
pub use self::tcp::Tcp;


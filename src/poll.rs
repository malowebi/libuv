use std::fmt;
use std::convert::From;
use bitflags::bitflags;

use crate::Loop;
use crate::error::Error;
use crate::handle::{Handle, HandleImpl};
use crate::bindings::{
  uv_poll_t, uv_poll_event, uv_poll_cb,
  uv_poll_init, uv_poll_start, uv_poll_stop,
  UV_READABLE, UV_WRITABLE, UV_DISCONNECT, UV_PRIORITIZED
};

pub type PollCb = uv_poll_cb;

repr!{
  pub type Poll = uv_poll_t;
}

bitflags! {
  pub struct PollEvent: uv_poll_event {
    const READABLE    = UV_READABLE;
    const WRITABLE    = UV_WRITABLE;
    const DISCONNECT  = UV_DISCONNECT;
    const PRIORITIZED = UV_PRIORITIZED;
  }
}

impl From<PollEvent> for uv_poll_event {
  fn from(evt: PollEvent) -> uv_poll_event {
    evt.bits as uv_poll_event
  }
}

impl From<PollEvent> for i32 {
  fn from(evt: PollEvent) -> Self {
    evt.bits as uv_poll_event as i32
  }
}

impl From<uv_poll_event> for PollEvent {
  fn from(i: uv_poll_event) -> Self {
    match i {
      UV_READABLE     => PollEvent::READABLE,
      UV_WRITABLE     => PollEvent::WRITABLE,
      UV_DISCONNECT   => PollEvent::DISCONNECT,
      UV_PRIORITIZED  => PollEvent::PRIORITIZED,
      _               => unreachable!()
    }
  }
}

impl Poll {
  pub fn init(&mut self, l: &mut Loop, fd: libc::c_int) -> Result<(), Error> {
    let rc = unsafe {
      uv_poll_init(l.as_mut_ptr(), self as *mut Poll, fd)
    };
    if rc < 0 {
      return Err( Error::from(rc) );
    }
    Ok(())
  }

  pub fn start(&mut self, events: PollEvent, cb: PollCb) -> Result<(), Error> {
    let res = unsafe {
      uv_poll_start(self as *mut Poll, events.into(), cb)
    };
    if res < 0 {
      return Err( Error::from(res) );
    }
    Ok(())
  }

  pub fn stop(&mut self) -> Result<(), Error> {
    let rc = unsafe {
      uv_poll_stop(self as *mut Poll)
    };
    if rc < 0 {
      return Err( Error::from(rc) );
    }
    Ok(())
  }
}

impl HandleImpl for Poll {
  fn as_handle(&self) -> &Handle {
    unsafe {
      let ptr = self as *const Poll as *const Handle;
      ptr.as_ref().unwrap()
    }
  }
  fn as_mut_handle(&mut self) -> &mut Handle {
    unsafe {
      let ptr = self as *mut Poll as *mut Handle;
      ptr.as_mut().unwrap()
    }
  }
}

impl fmt::Debug for Poll {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("Poll")
      .field("data",  &self.data)
      .field("loop",  &self.loop_)
      .field("type",  &self.type_)
      .field("close_cb",  &self.close_cb)
      .field("handle_queue",  &self.handle_queue)
      .field("next_closing",  &self.next_closing)
      .field("flags",  &self.flags)
      .field("poll_cb",  &self.poll_cb)
      .field("io_watcher",  &self.io_watcher)
      .finish()
  }
}

impl fmt::Display for uv_poll_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[test]
fn test_layout_poll() {
  use std::mem;
  assert_eq!(
    mem::size_of::<Poll>(),
    mem::size_of::<uv_poll_t>(),
    concat!("Size of: ", stringify!(Poll))
  );
  assert_eq!(
    mem::align_of::<Poll>(),
    mem::align_of::<uv_poll_t>(),
    concat!("Alignment of ", stringify!(Poll))
  );
  assert_eq!(
    mem::align_of::<Poll>(),
    mem::align_of::<Handle>(),
    concat!("Alignment of ", stringify!(Poll))
  );
}


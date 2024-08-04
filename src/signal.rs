use std::fmt;

use crate::r#loop::Loop;
use crate::error::Error;
use crate::stream::{Stream, StreamImpl};
use crate::handle::{Handle, HandleImpl};
use crate::bindings::{
  uv_signal_t,
  uv_signal_init, uv_signal_start, uv_signal_start_oneshot, uv_signal_stop
};

pub type SignalCb = crate::bindings::uv_signal_cb;

repr!{
  pub type Signal = uv_signal_t;
}

impl Signal {
  pub fn init(&mut self, l: &mut Loop) -> Result<(), Error> {
    let rc = unsafe {
      uv_signal_init(l.as_mut_ptr(), self.as_mut_ptr())
    };
    if rc < 0 {
      return Err( Error::from(rc) );
    }
    Ok(())
  }

  pub fn start(&mut self, cb: SignalCb, signum: i32) -> Result<(), Error> {
    let res = unsafe {
      uv_signal_start(self.as_mut_ptr(), cb, signum)
    };
    if res < 0 {
      return Err( Error::from(res) );
    }
    Ok(())
  }

  pub fn start_oneshot(&mut self, cb: SignalCb, signum: i32) -> Result<(), Error> {
    let res = unsafe {
      uv_signal_start_oneshot(self.as_mut_ptr(), cb, signum)
    };
    if res < 0 {
      return Err( Error::from(res) );
    }
    Ok(())
  }

  pub fn stop(&mut self) -> Result<(), Error> {
    let rc = unsafe {
      uv_signal_stop(self.as_mut_ptr())
    };
    if rc < 0 {
      return Err( Error::from(rc) );
    }
    Ok(())
  }
}

impl HandleImpl for Signal {
  fn as_handle(&self) -> &Handle {
    unsafe {
      let ptr = self.as_ptr() as *const Handle;
      ptr.as_ref().unwrap()
    }
  }
  fn as_mut_handle(&mut self) -> &mut Handle {
    unsafe {
      let ptr = self.as_mut_ptr() as *mut Handle;
      ptr.as_mut().unwrap()
    }
  }
}

impl StreamImpl for Signal {
  fn as_stream(&self) -> &Stream {
    unsafe {
      let ptr = self.as_ptr() as *const Stream;
      ptr.as_ref().unwrap()
    }
  }
  fn as_mut_stream(&mut self) -> &mut Stream {
    unsafe {
      let ptr = self.as_mut_ptr() as *mut Stream;
      ptr.as_mut().unwrap()
    }
  }
}

impl fmt::Debug for Signal {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("Signal")
      .field("data",  &self.data)
      .field("loop",  &self.loop_)
      .field("type",  &self.type_)
      .field("close_cb",  &self.close_cb)
      .field("handle_queue",  &self.handle_queue)
      .field("u",  unsafe { &self.u.fd })
      .field("next_closing",  &self.next_closing)
      .field("flags",  &self.flags)
      .field("signal_cb",  &self.signal_cb)
      .field("signum",  &self.signum)
      .field("tree_entry",  &self.tree_entry)
      .field("caught_signals",  &self.caught_signals)
      .field("dispatched_signals",  &self.dispatched_signals)
      .finish()
  }
}

#[test]
fn test_layout_signal() {
  use std::mem;
  assert_eq!(
    mem::size_of::<Signal>(),
    mem::size_of::<uv_signal_t>(),
    concat!("Size of: ", stringify!(Signal))
  );
  assert_eq!(
    mem::align_of::<Signal>(),
    mem::align_of::<uv_signal_t>(),
    concat!("Alignment of ", stringify!(Signal))
  );
  assert_eq!(
    mem::align_of::<Signal>(),
    mem::align_of::<Handle>(),
    concat!("Alignment of ", stringify!(Signal))
  );
  assert_eq!(
    mem::align_of::<Signal>(),
    mem::align_of::<Stream>(),
    concat!("Alignment of ", stringify!(Signal))
  );
}

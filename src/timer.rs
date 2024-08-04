use std::fmt;

use crate::bindings::{
  uv_timer_cb, uv_timer_t,
  uv_timer_init, uv_timer_start, uv_timer_stop
};

use crate::r#loop::Loop;
use crate::error::Error;
use crate::handle::{Handle, HandleImpl};

repr!{
  pub type Timer = uv_timer_t;
}

impl Timer {
  pub fn init(&mut self, l: &mut Loop) -> Result<(), Error> {
    let rc = unsafe {
      uv_timer_init(l.as_mut_ptr(), self.as_mut_ptr())
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }

  pub fn start(&mut self, timer_cb: uv_timer_cb, timeout: u64, repeat: u64) -> Result<(), Error> {
    let rc = unsafe {
      uv_timer_start(self.as_mut_ptr(), timer_cb, timeout, repeat)
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }

  pub fn stop(&mut self) -> Result<(), Error> {
    let rc = unsafe {
      uv_timer_stop(self.as_mut_ptr())
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }
}

impl HandleImpl for Timer {
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

impl fmt::Debug for Timer {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("Timer")
      .field("data",  &self.data)
      .field("loop",  &self.loop_)
      .field("type",  &self.type_)
      .field("close_cb",  &self.close_cb)
      .field("handle_queue",  &self.handle_queue)
      .field("next_closing",  &self.next_closing)
      .field("flags",  &self.flags)
      .field("timer_cb",  &self.timer_cb)
      .field("heap_node",  &self.heap_node)
      .field("timeout",  &self.timeout)
      .field("repeat",  &self.repeat)
      .field("start_id",  &self.start_id)
      .finish()
  }
}

impl fmt::Display for uv_timer_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[test]
fn test_layout_timer() {
  use std::mem;
  assert_eq!(
    mem::size_of::<Timer>(),
    mem::size_of::<uv_timer_t>(),
    concat!("Size of: ", stringify!(Timer))
  );
  assert_eq!(
    mem::align_of::<Timer>(),
    mem::align_of::<uv_timer_t>(),
    concat!("Alignment of ", stringify!(Timer))
  );
  assert_eq!(
    mem::align_of::<Timer>(),
    mem::align_of::<Handle>(),
    concat!("Alignment of ", stringify!(Timer))
  );
}


use std::fmt;

use crate::r#loop::Loop;
use crate::error::Error;
use crate::handle::{Handle, HandleImpl};
use crate::bindings::{
  uv_check_t, uv_check_cb,
  uv_check_init, uv_check_start, uv_check_stop
};

pub type CheckCb = uv_check_cb;

repr!{
  pub type Check = uv_check_t;
}

impl Check {
  pub fn init(&mut self, l: &mut Loop) -> Result<(), Error> {
    let rc = unsafe {
      uv_check_init(l.as_mut_ptr(), self.as_mut_ptr())
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }

  pub fn start(&mut self, cb: CheckCb) -> Result<(), Error> {
    let rc = unsafe {
      uv_check_start(self.as_mut_ptr(), cb)
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }

  pub fn stop(&mut self) -> Result<(), Error> {
    let rc = unsafe {
      uv_check_stop(self.as_mut_ptr())
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }
}

impl HandleImpl for Check {
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

impl fmt::Debug for Check {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("Check")
      .field("data", &self.data)
      .field("loop", &self.loop_)
      .field("type", &self.type_)
      .field("close_cb",  &self.close_cb)
      .field("handle_queue", &self.handle_queue)
      .field("next_closing", &self.next_closing)
      .field("flags", &self.flags)
      .field("check_cb", &self.check_cb)
      .field("queue",  &self.queue)
      .finish()
  }
}

impl fmt::Display for Check {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[test]
fn test_layout_check() {
  use std::mem;
  assert_eq!(
    mem::size_of::<Check>(),
    mem::size_of::<uv_check_t>(),
    concat!("Size of: ", stringify!(Check))
  );
  assert_eq!(
    mem::align_of::<Check>(),
    mem::align_of::<uv_check_t>(),
    concat!("Alignment of ", stringify!(Check))
  );
  assert_eq!(
    mem::align_of::<Check>(),
    mem::align_of::<Handle>(),
    concat!("Alignment of ", stringify!(Check))
  );
}

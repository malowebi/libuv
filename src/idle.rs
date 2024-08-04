use std::fmt;

use crate::bindings::{
  uv_idle_t, uv_idle_cb,
  uv_idle_init, uv_idle_start, uv_idle_stop
};

use crate::r#loop::Loop;
use crate::error::Error;
use crate::handle::{Handle, HandleImpl};

pub type IdleCb = uv_idle_cb;

repr!{
  pub type Idle = uv_idle_t;
}

impl Idle {
  pub fn init(&mut self, l: &mut Loop) -> Result<(), Error> {
    let rc = unsafe {
      uv_idle_init(l.as_mut_ptr(), self.as_mut_ptr())
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }

  pub fn start(&mut self, idle_cb: IdleCb) -> Result<(), Error> {
    let rc = unsafe {
      uv_idle_start(self.as_mut_ptr(), idle_cb)
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }

  pub fn stop(&mut self) -> Result<(), Error> {
    let rc = unsafe {
      uv_idle_stop(self.as_mut_ptr())
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }
}

impl HandleImpl for Idle {
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

impl fmt::Debug for Idle {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("Idle")
      .field("type",  &self.type_)
      .field("loop",  &self.loop_)
      .field("data",  &self.data)
      .field("flags",  &self.flags)
      .field("close_cb",  &self.close_cb)
      .field("idle_cb",  &self.idle_cb)
      .finish()
  }
}

impl fmt::Display for Idle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[test]
fn test_layout_idle() {
  use std::mem;
  assert_eq!(
    mem::size_of::<Idle>(),
    mem::size_of::<uv_idle_t>(),
    concat!("Size of: ", stringify!(Idle))
  );
  assert_eq!(
    mem::align_of::<Idle>(),
    mem::align_of::<uv_idle_t>(),
    concat!("Alignment of ", stringify!(Idle))
  );
  assert_eq!(
    mem::align_of::<Idle>(),
    mem::align_of::<Handle>(),
    concat!("Alignment of ", stringify!(Idle))
  );
}

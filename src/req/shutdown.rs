use std::fmt;

use crate::handle::Handle;
use crate::req::{Req, ReqImpl};
use crate::bindings::uv_shutdown_t;

pub type ShutdownCb = crate::bindings::uv_shutdown_cb;

repr!{
  pub type Shutdown = uv_shutdown_t;
}

impl Shutdown {
  pub fn handle(&self) -> &Handle {
    unsafe {
      let handle = self.handle as *const Handle;
      handle.as_ref().unwrap()
    }
  }
}

impl ReqImpl for Shutdown {
  fn as_request(&self) -> &Req {
    unsafe {
      let ptr = self.as_ptr() as *const Req;
      ptr.as_ref().unwrap()
    }
  }
  
  fn as_mut_request(&mut self) -> &mut Req {
    unsafe {
      let ptr = self.as_mut_ptr() as *mut Req;
      ptr.as_mut().unwrap()
    }
  }
}

// impl fmt::Debug for Shutdown {
//   fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
//     fmt
//       .debug_struct("Shutdown")
//       // UV_HANDLE_FIELDS
//       .field("data",  &self.data)
//       .field("type",  &self.type_)
//       .field("reserved",  &self.reserved)
//       // UV_SHUTDOWN_FIELDS
//       .field("handle",  &self.handle)
//       .field("cb",  &self.cb)
//       .finish()
//   }
// }

impl fmt::Display for Shutdown {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[test]
fn test_layout_shutdown() {
  use std::mem;
  use crate::bindings::uv_req_t;
  assert_eq!(
    mem::size_of::<Shutdown>(),
    mem::size_of::<uv_shutdown_t>(),
    concat!("Size of: ", stringify!(Shutdown))
  );
  assert_eq!(
    mem::align_of::<Shutdown>(),
    mem::align_of::<uv_shutdown_t>(),
    concat!("Alignment of ", stringify!(Shutdown))
  );
  assert_eq!(
    mem::align_of::<Shutdown>(),
    mem::align_of::<uv_req_t>(),
    concat!("Alignment of ", stringify!(Shutdown))
  );
}

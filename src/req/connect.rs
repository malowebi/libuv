use std::fmt;

use crate::handle::Handle;
use crate::req::{Req, ReqImpl};
use crate::bindings::uv_connect_t;

pub type ConnectCb = crate::bindings::uv_connect_cb;

repr!{
  pub type Connect = uv_connect_t;
}

impl Connect {
  pub fn handle(&self) -> &Handle {
    unsafe {
      let handle = self.handle as *const Handle;
      handle.as_ref().unwrap()
    }
  }
  
  pub fn handle_mut(&mut self) -> &mut Handle {
    unsafe {
      let handle = self.handle as *mut Handle;
      handle.as_mut().unwrap()
    }
  }
}

impl ReqImpl for Connect {
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

// impl fmt::Debug for Connect {
//   fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
//     fmt
//       .debug_struct("Connect")
//       // UV_REQ_COMMON_FIELDS
//       .field("data",  &self.data)
//       .field("type",  &self.type_)
//       .field("reserved",  &self.reserved)
//       // UV_CONNECT_FIELDS
//       .field("cb",  &self.cb)
//       .field("handle",  &self.handle)
//       // UV_CONNECT_PRIVATE_FIELDS
//       .field("queue",  &self.queue)
//       .finish()
//   }
// }

impl fmt::Display for Connect {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[test]
fn test_layout_connect() {
  use std::mem;
  use crate::bindings::uv_req_t;
  assert_eq!(
    mem::size_of::<Connect>(),
    mem::size_of::<uv_connect_t>(),
    concat!("Size of: ", stringify!(Connect))
  );
  assert_eq!(
    mem::align_of::<Connect>(),
    mem::align_of::<uv_connect_t>(),
    concat!("Alignment of ", stringify!(Connect))
  );
  assert_eq!(
    mem::align_of::<Connect>(),
    mem::align_of::<uv_req_t>(),
    concat!("Alignment of ", stringify!(Connect))
  );
}


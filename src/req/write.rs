use crate::handle::Handle;
use crate::req::{Req, ReqImpl};
use crate::bindings::uv_write_t;

pub type WriteCb = crate::bindings::uv_write_cb;

repr!{
  pub type Write = uv_write_t;
}

impl Write {
  pub fn handle(&self) -> &Handle {
    unsafe {
      let handle = self.handle as *const Handle;
      handle.as_ref().unwrap()
    }
  }
}

impl ReqImpl for Write {
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

#[test]
fn test_layout_write() {
  use std::mem;
  use crate::bindings::uv_req_t;
  assert_eq!(
    mem::size_of::<Write>(),
    mem::size_of::<uv_write_t>(),
    concat!("Size of: ", stringify!(Write))
  );
  assert_eq!(
    mem::align_of::<Write>(),
    mem::align_of::<uv_write_t>(),
    concat!("Alignment of ", stringify!(Write))
  );
  assert_eq!(
    mem::align_of::<Write>(),
    mem::align_of::<uv_req_t>(),
    concat!("Alignment of ", stringify!(Write))
  );
}

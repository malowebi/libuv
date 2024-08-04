use crate::r#loop::Loop;
use crate::error::Error;
use crate::handle::{Handle, HandleImpl};
use crate::bindings::{
  uv_prepare_t, uv_prepare_cb, 
  uv_prepare_init, uv_prepare_start, uv_prepare_stop,
};

repr!{
  pub type Prepare = uv_prepare_t;
}

pub type PrepareCb = uv_prepare_cb;

impl Prepare {
  pub fn init(&mut self, l: &mut Loop) -> Result<(), Error> {
    let rc = unsafe {
      uv_prepare_init(l.as_mut_ptr(), self.as_mut_ptr())
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(())
  }

  pub fn start(&mut self, cb: PrepareCb) -> Result<(), Error> {
    let rc = unsafe {
      uv_prepare_start(self.as_mut_ptr(), cb)
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(())
  }

  pub fn stop(&mut self) -> Result<(), Error> {
    let rc = unsafe {
      uv_prepare_stop(self.as_mut_ptr())
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(())
  }
}

impl HandleImpl for Prepare {
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


#[test]
fn test_layout_prepare() {
  use std::mem;
  assert_eq!(
    mem::size_of::<Prepare>(),
    mem::size_of::<uv_prepare_t>(),
    concat!("Size of: ", stringify!(Prepare))
  );
  assert_eq!(
    mem::align_of::<Prepare>(),
    mem::align_of::<uv_prepare_t>(),
    concat!("Alignment of ", stringify!(Prepare))
  );
  assert_eq!(
    mem::align_of::<Prepare>(),
    mem::align_of::<Handle>(),
    concat!("Alignment of ", stringify!(Prepare))
  );
}


  /*
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct uv_prepare_s {
    pub data: *mut ::std::os::raw::c_void,
    pub loop_: *mut uv_loop_t,
    pub type_: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut ::std::os::raw::c_void; 2usize],
    pub u: uv_prepare_s__bindgen_ty_1,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::std::os::raw::c_uint,
    pub prepare_cb: uv_prepare_cb,
    pub queue: [*mut ::std::os::raw::c_void; 2usize],
  }

  #[repr(C)]
  #[derive(Copy, Clone)]
  pub union uv_prepare_s__bindgen_ty_1 {
    pub fd: ::std::os::raw::c_int,
    pub reserved: [*mut ::std::os::raw::c_void; 4usize],
    _bindgen_union_align: [u64; 4usize],
  }
  */

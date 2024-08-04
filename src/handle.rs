use std::os::raw::c_void;

use crate::error::Error;
use crate::r#loop::Loop;
use crate::bindings::{
  uv_handle_t, uv_close_cb, uv_os_fd_t,
  UV_UNKNOWN_HANDLE, UV_ASYNC, UV_CHECK, UV_FS_EVENT, UV_FS_POLL, UV_HANDLE, 
  UV_IDLE, UV_NAMED_PIPE, UV_POLL, UV_PREPARE, UV_PROCESS, UV_STREAM, UV_TCP, 
  UV_TIMER, UV_TTY, UV_UDP, UV_SIGNAL, UV_FILE, UV_HANDLE_TYPE_MAX
};

pub type Fd       = uv_os_fd_t /* i32 */;
pub type CloseCb  = uv_close_cb /* Option<unsafe extern "C" fn(handle: *mut uv_handle_t)> */;

#[repr(u32)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum HandleType {
  UnknownHandle = /* 00 */ UV_UNKNOWN_HANDLE,
  Async         = /* 01 */ UV_ASYNC,
  Check         = /* 02 */ UV_CHECK,
  FsEvent       = /* 03 */ UV_FS_EVENT,
  FsPoll        = /* 04 */ UV_FS_POLL,
  Handle        = /* 05 */ UV_HANDLE,
  Idle          = /* 06 */ UV_IDLE,
  NamedPipe     = /* 07 */ UV_NAMED_PIPE,
  Poll          = /* 08 */ UV_POLL,
  Prepare       = /* 09 */ UV_PREPARE,
  Process       = /* 10 */ UV_PROCESS,
  Stream        = /* 11 */ UV_STREAM,
  Tcp           = /* 12 */ UV_TCP,
  Timer         = /* 13 */ UV_TIMER,
  Tty           = /* 14 */ UV_TTY,
  Udp           = /* 15 */ UV_UDP,
  Signal        = /* 16 */ UV_SIGNAL,
  File          = /* 17 */ UV_FILE,
  HandleTypeMax = /* 18 */ UV_HANDLE_TYPE_MAX,
}

impl From<u32> for HandleType {
  fn from(i: u32) -> Self {
    match i {
      UV_UNKNOWN_HANDLE   => /* 00 */ HandleType::UnknownHandle,
      UV_ASYNC            => /* 01 */ HandleType::Async,
      UV_CHECK            => /* 02 */ HandleType::Check,
      UV_FS_EVENT         => /* 03 */ HandleType::FsEvent,
      UV_FS_POLL          => /* 04 */ HandleType::FsPoll,
      UV_HANDLE           => /* 05 */ HandleType::Handle,
      UV_IDLE             => /* 06 */ HandleType::Idle,
      UV_NAMED_PIPE       => /* 07 */ HandleType::NamedPipe,
      UV_POLL             => /* 08 */ HandleType::Poll,
      UV_PREPARE          => /* 09 */ HandleType::Prepare,
      UV_PROCESS          => /* 10 */ HandleType::Process,
      UV_STREAM           => /* 11 */ HandleType::Stream,
      UV_TCP              => /* 12 */ HandleType::Tcp,
      UV_TIMER            => /* 13 */ HandleType::Timer,
      UV_TTY              => /* 14 */ HandleType::Tty,
      UV_UDP              => /* 15 */ HandleType::Udp,
      UV_SIGNAL           => /* 16 */ HandleType::Signal,
      UV_FILE             => /* 17 */ HandleType::File,
      UV_HANDLE_TYPE_MAX  => /* 18 */ HandleType::HandleTypeMax,
      _                   => unreachable!()
    }
  }
}

impl From<HandleType> for u32 {
  fn from(r#type: HandleType) -> Self {
    r#type as u32
  }
}

repr!{
  pub type Handle = uv_handle_t;
}

pub trait HandleImpl {
  fn as_handle(&self) -> &Handle;
  fn as_mut_handle(&mut self) -> &mut Handle;

  #[inline]
  fn size(&self) -> usize {
    use crate::bindings::*;
    unsafe {
      uv_handle_size(self.get_type().into()) as usize
    }
  }

  fn get_loop(&self) -> &Loop {
    use crate::bindings::*;
    unsafe {
      let handle = self.as_handle().as_ptr();
      let pointer = uv_handle_get_loop(handle) as *const Loop;
      pointer.as_ref().unwrap()
    }
  }

  fn get_mut_loop(&mut self) -> &mut Loop {
    use crate::bindings::*;
    unsafe {
      let handle = self.as_mut_handle().as_mut_ptr();
      let pointer = uv_handle_get_loop(handle) as *mut Loop;
      pointer.as_mut().unwrap()
    }
  }

  #[inline]
  fn get_type(&self) -> HandleType {
    use crate::bindings::*;
    let rc = unsafe {
      uv_handle_get_type(self.as_handle().as_ptr())
    };
    HandleType::from(rc)
  }

  #[inline]
  fn get_data(&self) -> *mut c_void {
    use crate::bindings::*;
    unsafe {
      uv_handle_get_data(self.as_handle().as_ptr())
    }
  }

  #[inline]
  fn set_data<T>(&mut self, data: *mut T) {
    use crate::bindings::*;
    unsafe {
      uv_handle_set_data(self.as_mut_handle().as_mut_ptr(), data as *mut c_void)
    }
  }  

  fn clear_data(&mut self) -> *mut c_void {
    let data = self.get_data();
    self.set_data( std::ptr::null_mut() as *mut c_void );
    data
  }

  fn is_active(&mut self) -> bool {
    use crate::bindings::*;
    1 == unsafe {
      uv_is_active(self.as_handle().as_ptr())
    }
  }

  fn is_closing(&self) -> bool {
    use crate::bindings::*;
    1 == unsafe {
      uv_is_closing(self.as_handle().as_ptr())
    }
  }

  fn close(&mut self, close_cb: CloseCb) {
    use crate::bindings::*;
    let handle = self.as_mut_handle();
    unsafe {
      uv_close(handle.as_mut_ptr(), close_cb)
    }
  }

  fn fileno(&self) -> Result<Fd, Error> {
    use crate::bindings::*;

    unsafe {
      let mut fd: Fd = 0;
      let rc = uv_fileno(self.as_handle().as_ptr(), &mut fd as *mut Fd);
      
      if rc < 0 {
        return Err(Error::from(rc));
      }

      Ok(fd)
    }
  }
}

impl HandleImpl for Handle {
  #[inline(always)]
  fn as_handle(&self) -> &Handle {
    self
  }
  #[inline(always)]
  fn as_mut_handle(&mut self) -> &mut Handle {
    self
  }
}

#[test]
fn test_layout_handle() {
  use std::mem;
  assert_eq!(
    mem::size_of::<Handle>(),
    mem::size_of::<uv_handle_t>(),
    concat!("Size of: ", stringify!(Handle))
  );
  assert_eq!(
    mem::align_of::<Handle>(),
    mem::align_of::<uv_handle_t>(),
    concat!("Alignment of ", stringify!(Handle))
  );
}

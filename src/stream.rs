use std::os::raw::{c_int, c_uint};

use crate::buf::Buf;
use crate::error::Error;
use crate::handle::{Handle, HandleImpl};
use crate::req::{Write, WriteCb, Shutdown, ShutdownCb};
use crate::bindings::{
  uv_stream_t,
  uv_is_readable, uv_is_writable, uv_stream_set_blocking,
  uv_listen, uv_accept, uv_read_start, uv_read_stop,
  uv_write, uv_shutdown
};

pub type ReadCb = crate::bindings::uv_read_cb;
pub type AllocCb = crate::bindings::uv_alloc_cb;
pub type ConnectionCb = crate::bindings::uv_connection_cb;

repr!{
  pub type Stream = uv_stream_t;
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub trait StreamImpl: HandleImpl {
  fn as_stream(&self) -> &Stream;
  fn as_mut_stream(&mut self) -> &mut Stream;

  // fn get_mut_loop(&mut self) -> &Loop {
  //   unsafe {
  //     let ptr = self.as_mut_stream().loop_ as *mut Loop;
  //     ptr.as_mut().unwrap()
  //   }
  // }

  fn is_readable(&self) -> bool {
    let rc = unsafe {
      uv_is_readable(self.as_stream().as_ptr())
    };
    matches!(rc, 1)
  }

  fn is_writable(&self) -> bool {
    let rc = unsafe {
      uv_is_writable(self.as_stream().as_ptr())
    };
    matches!(rc, 1)
  }  

  fn set_blocking(&mut self, blocking: bool) -> Result<(), Error> {
    let rc = unsafe {
      uv_stream_set_blocking(self.as_mut_stream().as_mut_ptr(), if blocking { 1 } else { 0 })
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }

  fn listen(&mut self, max_connections: c_int, connection_cb: ConnectionCb) -> Result<(), Error> {
    let rc = unsafe {
      uv_listen(self.as_mut_stream().as_mut_ptr(), max_connections, connection_cb)
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }

  /// Accept incoming connections from the `ConnectionCb` passed to `listen()`.
  fn accept(&mut self, client: &mut Stream) -> Result<(), Error> {
    let rc = unsafe {
      uv_accept(self.as_mut_stream().as_mut_ptr(), client as *mut Stream as *mut uv_stream_t)
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }

  fn read_start(&mut self, alloc_cb: AllocCb, read_cb: ReadCb) -> Result<(), Error> {
    let rc = unsafe {
      uv_read_start(self.as_mut_stream().as_mut_ptr(), alloc_cb, read_cb)
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }

  fn read_stop(&mut self) -> Result<(), Error> {
    let rc = unsafe {
      uv_read_stop(self.as_mut_stream().as_mut_ptr())
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }

  fn write(&mut self, bufs: *const Buf, nbufs: c_uint, write_cb: WriteCb) -> Result<(), Error> {
    let rc = unsafe {
      let req = Box::into_raw(Write::boxed()) as *mut Write;
      uv_write(req, self.as_mut_stream().as_mut_ptr(), bufs, nbufs, write_cb)
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }

  fn shutdown(&mut self, shutdown_cb: ShutdownCb) -> Result<(), Error> {
    let rc = unsafe {
      let req = Box::into_raw(Shutdown::boxed()) as *mut Shutdown;
      uv_shutdown(req, self.as_mut_stream().as_mut_ptr(), shutdown_cb)
    };
    if rc < 0 {
      return Err(Error::from(rc));
    }
    Ok(())
  }  
}

impl HandleImpl for Stream  {
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

impl StreamImpl for Stream  {
  #[inline(always)]
  fn as_stream(&self) -> &Stream {
    self
  }
  #[inline(always)]
  fn as_mut_stream(&mut self) -> &mut Stream {
    self
  }
}

#[test]
fn test_layout_stream() {
  use std::mem;
  assert_eq!(
    mem::size_of::<Stream>(),
    mem::size_of::<uv_stream_t>(),
    concat!("Size of: ", stringify!(Stream))
  );
  assert_eq!(
    mem::align_of::<Stream>(),
    mem::align_of::<uv_stream_t>(),
    concat!("Alignment of ", stringify!(Stream))
  );
  assert_eq!(
    mem::align_of::<Stream>(),
    mem::align_of::<Handle>(),
    concat!("Alignment of ", stringify!(Stream))
  );
}

use std::ops::BitOr;
use std::convert::{From, Into};

use crate::bindings::{
  sockaddr, uv_os_sock_t,
  uv_tcp_t,
  uv_connect_cb, uv_stdio_flags,
};

use crate::r#loop::Loop;
use crate::error::Error;
use crate::req::Connect;
use crate::stream::{Stream, StreamImpl};
use crate::handle::{Handle, HandleImpl};

pub use crate::bindings::{
  UV_READABLE, UV_WRITABLE,
};

pub type SockFd     = uv_os_sock_t;
pub type SockAddr   = sockaddr;
pub type ConnectFn  = uv_connect_cb;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TcpFlags {
  Readable = UV_READABLE,
  Writable = UV_WRITABLE,
}

repr!{
  pub type Tcp = uv_tcp_t;
  /*
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct uv_tcp_s {
    pub data: *mut ::std::os::raw::c_void,
    pub loop_: *mut uv_loop_t,
    pub type_: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut ::std::os::raw::c_void; 2usize],
    pub u: uv_tcp_s__bindgen_ty_1,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::std::os::raw::c_uint,
    pub write_queue_size: size_t,
    pub alloc_cb: uv_alloc_cb,
    pub read_cb: uv_read_cb,
    pub connect_req: *mut uv_connect_t,
    pub shutdown_req: *mut uv_shutdown_t,
    pub io_watcher: uv__io_t,
    pub write_queue: [*mut ::std::os::raw::c_void; 2usize],
    pub write_completed_queue: [*mut ::std::os::raw::c_void; 2usize],
    pub connection_cb: uv_connection_cb,
    pub delayed_error: ::std::os::raw::c_int,
    pub accepted_fd: ::std::os::raw::c_int,
    pub queued_fds: *mut ::std::os::raw::c_void,
    pub select: *mut ::std::os::raw::c_void,
    pub ipc: ::std::os::raw::c_int,
    pub tcp_fname: *const ::std::os::raw::c_char,
  }

  #[repr(C)]
  #[derive(Copy, Clone)]
  pub union uv_tcp_s__bindgen_ty_1 {
      pub fd: ::std::os::raw::c_int,
      pub reserved: [*mut ::std::os::raw::c_void; 4usize],
      _bindgen_union_align: [u64; 4usize],
  }
  */  
}

impl HandleImpl for Tcp {
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

impl StreamImpl for Tcp {
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

impl Tcp {
  /// Initialize a tcp handle. 
  pub fn init(&mut self, l: &mut Loop) -> Result<(), Error> {
    use crate::bindings::*;

    let rc = unsafe {
      uv_tcp_init(l.as_mut_ptr(), self.as_mut_ptr())
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(())
  }

  /// Open an existing file descriptor or SOCKET as a TCP handle.
  pub fn open(&mut self, sock: SockFd) -> Result<(), Error> {
    use crate::bindings::*;

    let rc = unsafe {
      uv_tcp_open(self.as_mut_ptr(), sock)
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(())
  }

  /// Enable TCP_NODELAY, which disables Nagle’s algorithm.
  pub fn nodelay(&mut self, enable: bool) -> Result<(), Error> {
    use crate::bindings::*;

    let rc = unsafe {
      uv_tcp_nodelay(self.as_mut_ptr(), if enable { 1 } else { 0 })
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(())
  }

  /// Bind the tcp to a file path (Unix) or a name (Windows).
  pub fn bind(&mut self, addr: &SockAddr) -> Result<(), Error> {
    use crate::bindings::*;

    let rc = unsafe {
      uv_tcp_bind(self.as_mut_ptr(), addr as *const SockAddr, 0)
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(())
  }

  /// Connect to the Unix domain socket or the named tcp.
  pub fn connect(&mut self, addr: &SockAddr, cb: ConnectFn) -> Result<(), Error> {
    use crate::bindings::*;

    let mut req = Connect::boxed();
    let req_ptr = req.as_mut_ptr();
    std::mem::forget(req);

    unsafe {
      uv_tcp_connect(req_ptr, self.as_mut_ptr(), addr as *const SockAddr, cb)
    };

    Ok(())
  }

  /// Get the name of the Unix domain socket or the named tcp.
  ///
  /// A preallocated `buffer` must be provided. The `size` parameter 
  /// holds the length of the `buffer` and it’s set to the number of bytes 
  /// written to the `buffer` on output. If the `buffer` is not big enough, 
  /// UV_ENOBUFS will be returned  and `size` will contain the required size.
  // pub fn getsockname(&mut self, buffer: &mut[u8], size: &mut size_t) -> Result<(), Error> {
  pub fn getsockname(&mut self, buffer: &mut[u8]) -> Result<usize, Error> {
    use crate::bindings::*;

    let mut name = sockaddr_storage::default();
    let mut size = std::mem::size_of::<sockaddr_storage>() as i32;
    let name_ptr = &mut name as *mut sockaddr_storage as *mut SockAddr;

    let rc = unsafe {
      uv_tcp_getsockname(self.as_ptr(), name_ptr, &mut size)
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    let src = name_ptr as *const sockaddr_in;
    let dst = buffer.as_mut_ptr() as *mut c_char;

    let rc = unsafe {
      uv_ip4_name(src, dst, size as size_t)
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(size as usize)
  }

  /// Get the name of the Unix domain socket or the named tcp.
  ///
  /// A preallocated `buffer` must be provided. The `size` parameter 
  /// holds the length of the `buffer` and it’s set to the number of bytes 
  /// written to the `buffer` on output. If the `buffer` is not big enough, 
  /// UV_ENOBUFS will be returned  and `size` will contain the required size.  
  pub fn getpeername(&mut self, buffer: &mut[u8]) -> Result<usize, Error> {
    use crate::bindings::*;

    let mut name = sockaddr_storage::default();
    let mut size = std::mem::size_of::<sockaddr_storage>() as i32;
    let name_ptr = &mut name as *mut sockaddr_storage as *mut SockAddr;

    let rc = unsafe {
      uv_tcp_getpeername(self.as_ptr(), name_ptr, &mut size)
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    let src = name_ptr as *const sockaddr_in;
    let dst = buffer.as_mut_ptr() as *mut c_char;

    let rc = unsafe {
      uv_ip4_name(src, dst, size as size_t)
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(size as usize)
  }
}

impl From<TcpFlags> for uv_stdio_flags {
  fn from(flags: TcpFlags) -> uv_stdio_flags {
    flags as uv_stdio_flags
  }
}

impl From<uv_stdio_flags> for TcpFlags {
  fn from(i: uv_stdio_flags) -> Self {
    match i {
      UV_READABLE => TcpFlags::Readable,
      UV_WRITABLE => TcpFlags::Writable,
      _               => unreachable!()
    }
  }
}

impl BitOr for TcpFlags {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self::Output {
    let l: uv_stdio_flags = self.into();
    let r: uv_stdio_flags = rhs.into();
    Self::from(l | r)
  }
}


#[test]
fn test_algnmt_repr() {
  use std::mem;
  
  assert_eq!(
    mem::align_of::<Tcp>(),
    mem::align_of::<Stream>(),
    concat!("Alignment of ", stringify!(Tcp))
  );
  
  assert_eq!(
    mem::align_of::<Tcp>(),
    mem::align_of::<Handle>(),
    concat!("Alignment of ", stringify!(Tcp))
  );
}

use std::fmt;
use std::convert::{From, Into};
use std::os::raw::{c_int, c_char};

use crate::{HandleType, ReqImpl};

use crate::r#loop::Loop;
use crate::error::Error;
use crate::poll::PollEvent;
use crate::req::{Connect, ConnectCb};
use crate::stream::{Stream, StreamImpl};
use crate::handle::{Handle, HandleImpl};
use crate::bindings::{
  uv_pipe_t, uv_poll_event, size_t, uv_file,
  uv_pipe_init, uv_pipe_connect,
  uv_pipe_getsockname, uv_pipe_getpeername,
  uv_pipe_bind, uv_pipe_pending_type, uv_pipe_chmod,
  uv_pipe_pending_count, uv_pipe_open,
};

pub type Fd = uv_file;

repr!{
  pub type Pipe = uv_pipe_t;
}

impl HandleImpl for Pipe {
  fn as_handle(&self) -> &Handle {
    unsafe {
      let ptr = self as *const Pipe as *const Handle;
      ptr.as_ref().unwrap()
    }
  }

  fn as_mut_handle(&mut self) -> &mut Handle {
    unsafe {
      let ptr = self as *mut Pipe as *mut Handle;
      ptr.as_mut().unwrap()
    }
  }
}

impl StreamImpl for Pipe {
  fn as_stream(&self) -> &Stream {
    unsafe {
      let ptr = self as *const Pipe as *const Stream;
      ptr.as_ref().unwrap()
    }
  }

  fn as_mut_stream(&mut self) -> &mut Stream {
    unsafe {
      &mut *(self as *mut Pipe as *mut Stream)
    }
  }
}

impl Pipe {
  /// Initialize a pipe handle. 
  /// The ipc argument is a boolean to indicate if this pipe will be used 
  /// for handle passing between processes (which may change the bytes on the wire). 
  /// Only a connected pipe that will be passing the handles should have this flag set, 
  /// not the listening pipe that uv_accept is called on.
  pub fn init(&mut self, l: &mut Loop, ipc: bool) -> Result<(), Error> {
    let rc = unsafe {
      uv_pipe_init(l.as_mut_ptr(), self as *mut Pipe, if ipc { 1 } else { 0 })
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(())
  }

  /// Open an existing file descriptor (unix) or HANDLE (windows) as a pipe.
  /// The file descriptor is set to non-blocking mode.
  pub fn open(&mut self, fd: Fd) -> Result<(), Error> {
    let rc = unsafe {
      uv_pipe_open(self as *mut Pipe, fd)
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(())
  }

  /// Bind the pipe to a file path (Unix) or a name (Windows).
  pub fn bind(&mut self, name: &str) -> Result<(), Error> {
    let mut buf = String::with_capacity(name.len() + 1);
    buf.push_str(name);
    buf.push('\0');

    let rc = unsafe {
      uv_pipe_bind(self as *mut Pipe, buf.as_mut_ptr() as *const c_char)
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(())
  }

  /// Connect to the Unix domain socket or the named pipe.
  pub fn connect(&mut self, name: &str, cb: ConnectCb) -> Result<(), Error> {
    let mut req = Connect::boxed();
    req.set_data(self as *mut Self as *mut libc::c_void);

    let req_ptr = req.as_mut_ptr();

    let _ = Box::into_raw(req);

    let mut buf = String::with_capacity(name.len() + 1);
    buf.push_str(name);
    buf.push('\0');

    unsafe {
      uv_pipe_connect(req_ptr, self as *mut Pipe, buf.as_mut_ptr() as *const c_char, cb)
    };

    Ok(())
  }

  /// Get the name of the Unix domain socket or the named pipe.
  ///
  /// A preallocated `buffer` must be provided. The `size` parameter 
  /// holds the length of the `buffer` and it’s set to the number of bytes 
  /// written to the `buffer` on output. If the `buffer` is not big enough, 
  /// UV_ENOBUFS will be returned  and `size` will contain the required size.
  pub fn getsockname(&mut self, buffer: &mut[u8], size: &mut isize) -> Result<(), Error> {
    buffer.fill(0);

    let mut sz = buffer.len() as size_t;

    let rc = unsafe {
      uv_pipe_getsockname(
        self as *mut Pipe, 
        buffer.as_mut_ptr() as *mut c_char, 
        &mut sz as *mut size_t
      )
    };

    debug_assert!(isize::MAX as size_t >= sz);
    *size = sz as isize;    

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(())
  }

  /// Get the name of the Unix domain socket or the named pipe to which the handle is connected.
  ///
  /// A preallocated `buffer` must be provided. The `size` parameter 
  /// holds the length of the `buffer` and it’s set to the number of bytes 
  /// written to the `buffer` on output. If the `buffer` is not big enough, 
  /// UV_ENOBUFS will be returned  and `size` will contain the required size.  
  pub fn getpeername(&mut self, buffer: &mut[u8], size: &mut isize) -> Result<(), Error> {
    buffer.fill(0);

    let mut sz = buffer.len() as size_t;

    let rc = unsafe {
      uv_pipe_getpeername(
        self as *mut Pipe, 
        buffer.as_mut_ptr() as *mut c_char, 
        &mut sz as *mut size_t
      )
    };

    debug_assert!(isize::MAX as size_t >= sz);
    *size = sz as isize;    

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(())
  }

  /// Alters pipe permissions, allowing it to be accessed from processes run by different users. 
  /// Makes the pipe writable or readable by all users. 
  /// Mode can be UV_WRITABLE, UV_READABLE or UV_WRITABLE | UV_READABLE. 
  /// This function is blocking.
  pub fn chmod(&mut self, flags: PollEvent) -> Result<(), Error> {
    let rc = unsafe {
      let flags: uv_poll_event = flags.into();
      uv_pipe_chmod(self as *mut Pipe, flags as c_int)
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }
    
    Ok(())
  }

  pub fn pending_count(&mut self) -> Result<u32, Error> {
    let rc = unsafe {
      uv_pipe_pending_count(self as *mut Pipe)
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }
    
    Ok(rc as u32)
  }

  pub fn pending_type(&mut self) -> Result<HandleType, Error> {
    let ty = unsafe {
      uv_pipe_pending_type(self as *mut Pipe)
    };
    
    Ok( HandleType::from(ty) )
  }
}

impl fmt::Debug for Pipe {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("Pipe")
      // UV_HANDLE_FIELDS
      .field("data",  &self.data)
      .field("loop",  &self.loop_)
      .field("type",  &self.type_)
      .field("close_cb",  &self.close_cb)
      .field("handle_queue",  &self.handle_queue)
      .field("next_closing",  &self.next_closing)
      .field("flags",  &self.flags)
      // UV_STREAM_FIELDS
      .field("write_queue_size",  &self.write_queue_size)
      .field("alloc_cb",  &self.alloc_cb)
      .field("read_cb",  &self.read_cb)
      .field("connect_req",  &self.connect_req)
      .field("shutdown_req",  &self.shutdown_req)
      .field("write_queue",  &self.write_queue)
      .field("write_completed_queue",  &self.write_completed_queue)
      .field("connection_cb",  &self.connection_cb)
      .field("delayed_error",  &self.delayed_error)
      .field("accepted_fd",  &self.accepted_fd)
      .field("queued_fds",  &self.queued_fds)
      // UV_PIPE_FIELDS
      .field("ipc",  &self.ipc)
      // UV_PIPE_PRIVATE_FIELDS
      .field("pipe_fname",  &self.pipe_fname)
      .finish()
  }
}

impl fmt::Display for uv_pipe_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[test]
fn test_layout_pipe() {
  use std::mem;
  assert_eq!(
    mem::align_of::<Pipe>(),
    mem::align_of::<Stream>(),
    concat!("Alignment of ", stringify!(Pipe))
  );
  assert_eq!(
    mem::align_of::<Pipe>(),
    mem::align_of::<Handle>(),
    concat!("Alignment of ", stringify!(Pipe))
  );
}

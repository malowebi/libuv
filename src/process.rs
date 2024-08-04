use std::fmt;
use std::ffi::CStr;
// use std::io::Write;
// use std::io::Cursor;

pub type Char = libc::c_char;

use crate::error::Error;
use crate::r#loop::Loop;
use crate::handle::{Handle, HandleImpl};
use crate::stdio::StdioContainer;
use crate::bindings::{
  uv_process_kill, uv_spawn,
  uv_uid_t, uv_gid_t, uv_process_options_t, uv_process_t
};

repr!{
  pub type ProcessOptions = uv_process_options_t;
}

repr!{
  pub type Process = uv_process_t;
}

// pub type ExitCb  = crate::bindings::uv_exit_cb;
// pub type CloseCb = crate::bindings::uv_close_cb;
pub type ExitCb  = Option<unsafe extern "C" fn(proc: *mut Process, exit_status: i64, term_signal: i32)>;
pub type CloseCb = Option<unsafe extern "C" fn(handle: *mut Handle)>;

impl HandleImpl for Process {
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

impl Process {
  pub fn kill(&mut self, signum: i32) -> Result<(), Error> {
    let rc = unsafe {
      uv_process_kill(self.as_mut_ptr(), signum)
    };

    if rc < 0 {
      return Err( Error::from(rc) );
    }

    Ok(())
  }

  pub fn spawn(&mut self, l: &mut Loop, options: ProcessOptions) -> Result<(), Error> {
    let rc = unsafe {
      uv_spawn(l as *mut Loop, self.as_mut_ptr(), &options)
    };

    if rc < 0 {
      return Err(Error::from(rc));
    }

    Ok(())
  }
}

impl ProcessOptions {
  pub fn builder() -> ProcessOptionsBuilder {
    ProcessOptionsBuilder::default()
  }

  pub fn set_uid(mut self, uid: uv_uid_t) {
    self.uid = uid;
  }

  pub fn set_gid(mut self, gid: uv_gid_t) {
    self.gid = gid;
  }

  pub fn set_cwd(mut self, cwd: &str) -> Self {
    let cwd  = CStr::from_bytes_with_nul(cwd.as_bytes()).unwrap().as_ptr();
    self.cwd = cwd;
    self
  }

  pub fn set_file(mut self, file: &str) -> Self {
    let file  = CStr::from_bytes_with_nul(file.as_bytes()).unwrap().as_ptr();
    self.file = file;
    self
  }

  pub fn set_args(mut self, argv: &mut [String]) -> Self {
    use std::mem::ManuallyDrop;

    let mut args: Vec<*mut Char> = Vec::with_capacity(argv.len());

    for (_, arg) in argv.iter_mut().enumerate() {
      if arg.len() == 0 {
        args.push(std::ptr::null_mut());
        break;
      }
      if !arg.ends_with('\0') {
        arg.push('\0');
      }
      args.push(arg.as_mut_ptr() as *mut i8);
    }

    let mut args = ManuallyDrop::new(args);
    self.args = args.as_mut_ptr();
    self
  }

  pub fn set_stdio(mut self, stdio: &mut [StdioContainer]) -> Self {
    self.stdio = stdio.as_mut_ptr();
    self.stdio_count = stdio.len().try_into().expect("Invalid `stdio` option");
    self
  }
}

#[derive(Default)]
pub struct ProcessOptionsBuilder {
  options: ProcessOptions,
  // buffer:  Cursor<Vec<u8>>,
  // uid: uv_uid_t,
  // gid: uv_gid_t,
  // cwd: Option<*const c_char>,
  // file: Option<*const c_char>,
  // args: Vec<String>,
  // stdio: [StdioContainer; 3],
}

// impl Default for ProcessOptionsBuilder {
//   #[inline]
//   fn default() -> Self {
//     Self {
//       options: ProcessOptions::default(),
//     }
//   }
// }

impl ProcessOptionsBuilder {
  pub fn uid(mut self, uid: uv_uid_t) -> Self {
    self.options.uid = uid;
    self
  }

  pub fn gid(mut self, gid: uv_gid_t) -> Self {
    self.options.gid = gid;
    self
  }

  pub fn cwd(mut self, cwd: &str) -> Self {
    let cwd  = CStr::from_bytes_with_nul(cwd.as_bytes()).unwrap().as_ptr();
    self.options.cwd = cwd;
    self
  }

  pub fn file(mut self, file: &str) -> Self {
    let file  = CStr::from_bytes_with_nul(file.as_bytes()).unwrap().as_ptr();
    self.options.file = file;
    self
  }

  // pub fn args(mut self, args: &[String]) -> Self {
  pub fn args(self, _: &[String]) -> Self {
    // let buf = self.buffer.get_mut();

    // for arg in args {
    //   buf.write_all(arg.as_bytes()).unwrap();
    //   buf.write_all(&[0]).unwrap();
    // }

    // buf.flush().unwrap();

    // self.options.args = self.buffer.get_mut().as_mut_ptr() as *mut *mut i8;

    self
  }

  pub fn stdio(mut self, stdio: &mut [StdioContainer; 3]) -> Self {
    self.options.stdio = stdio.as_mut_ptr();
    self
  }

  pub fn build(self) -> ProcessOptions {
    self.options
  } 
}


impl fmt::Debug for Process {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("Process")
      .field("data",  &self.data)
      .field("type",   &self.type_)
      .field("flags",   &self.flags)
      .field("exit_cb",   &self.exit_cb)
      .field("pid",   &self.pid)
      .field("status",   &self.status)
      .finish()
  }
}

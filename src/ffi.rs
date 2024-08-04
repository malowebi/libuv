#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

use std::fmt;
use std::default::Default;
use std::bindings::{CString, CStr};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

pub use libc::{
  c_char, c_short, c_long, c_int, c_uint,
  size_t, ssize_t, c_void, gid_t, uid_t,
  termios, tcflag_t, cc_t, speed_t, 
  c_ulong,
  pthread_mutex_t, pthread_rwlock_t,
  FILE, NCCS,
};

pub fn cstr_to_str<'a>(cstr: *const c_char) -> &'a str {
  unsafe {
    let size = libc::strlen(cstr);
    let data = cstr as *const u8;
    let data = std::slice::from_raw_parts(data, size);
    std::str::from_utf8(data).unwrap()
  }
}

// pub fn str_to_cstr(s: &str) -> *const c_char {
//   let s = s.as_bytes();
//   let s = s.as_ptr();
//   s as *const c_char
// }

// pub fn string_to_cstr(s: &mut String) -> *const c_char {  
//   if s.chars().last().unwrap() != '\0' {
//     s.push('\0');
//   }
//   str_to_cstr( s.as_str() )
// }

pub(crate) fn str_to_cstr(s: &str) -> *const c_char {
  CStr::from_bytes_with_nul(s.as_bytes()).unwrap().as_ptr()
  // let s = s.as_bytes();
  // let s = s.as_ptr();
  // s as *const c_char
}

pub(crate) fn string_to_cstr(s: &mut String) -> *const c_char {
  if s.is_empty() || s.chars().last().unwrap() != '\0' {
    s.push('\0');
  }
  str_to_cstr( s.as_str() )
}

pub type  uv_fd_t           = c_int;
pub type  uv_gid_t          = gid_t;
pub type  uv_uid_t          = uid_t;
pub type  uv_errno_t        = c_int;
pub type  uv_file           = c_int;
pub type  uv_pid_t          = c_int;
pub type  uv_os_fd_t        = c_int;
pub type  uv_os_sock_t      = c_int;
pub type  uv_poll_event     = c_int;
pub type  uv_loop_option    = c_uint;
pub type  uv_run_mode       = c_uint;
pub type  uv_req_type       = c_uint;
pub type  uv_stdio_flags    = c_int;
pub type  uv_tcp_flags      = c_uint;
pub type  uv_process_flags  = c_uint;
pub type  uv_membership     = c_uint;
pub type  uv_handle_type    = c_uint;
pub type  uv_mutex_t        = pthread_mutex_t;
pub type  uv_rwlock_t       = pthread_rwlock_t;
pub type  sa_family_t       = c_uint;

pub const UV_UNKNOWN_HANDLE:  uv_handle_type = 0;
pub const UV_ASYNC:           uv_handle_type = 1;
pub const UV_CHECK:           uv_handle_type = 2;
pub const UV_FS_EVENT:        uv_handle_type = 3;
pub const UV_FS_POLL:         uv_handle_type = 4;
pub const UV_HANDLE:          uv_handle_type = 5;
pub const UV_IDLE:            uv_handle_type = 6;
pub const UV_NAMED_PIPE:      uv_handle_type = 7;
pub const UV_POLL:            uv_handle_type = 8;
pub const UV_PREPARE:         uv_handle_type = 9;
pub const UV_PROCESS:         uv_handle_type = 10;
pub const UV_STREAM:          uv_handle_type = 11;
pub const UV_TCP:             uv_handle_type = 12;
pub const UV_TIMER:           uv_handle_type = 13;
pub const UV_TTY:             uv_handle_type = 14;
pub const UV_UDP:             uv_handle_type = 15;
pub const UV_SIGNAL:          uv_handle_type = 16;
pub const UV_FILE:            uv_handle_type = 17;
pub const UV_HANDLE_TYPE_MAX: uv_handle_type = 18;

pub const UV__EOF:            c_int = -4095;
pub const UV__UNKNOWN:        c_int = -4094;
pub const UV__EAI_ADDRFAMILY: c_int = -3000;

pub const UV_TCP_IPV6ONLY: uv_tcp_flags = 1;

pub const UV_LEAVE_GROUP: uv_membership = 0;
pub const UV_JOIN_GROUP:  uv_membership = 1;

pub const UV_LOOP_BLOCK_SIGNAL: uv_loop_option = 0;
pub const UV_METRICS_IDLE_TIME: uv_loop_option = 1;

pub const UV_RUN_DEFAULT: uv_run_mode = 0;
pub const UV_RUN_ONCE:    uv_run_mode = 1;
pub const UV_RUN_NOWAIT:  uv_run_mode = 2;

pub const UV_UNKNOWN_REQ:       uv_req_type = 0;
pub const UV_REQ:               uv_req_type = 1;
pub const UV_CONNECT:           uv_req_type = 2;
pub const UV_WRITE:             uv_req_type = 3;
pub const UV_SHUTDOWN:          uv_req_type = 4;
pub const UV_UDP_SEND:          uv_req_type = 5;
pub const UV_FS:                uv_req_type = 6;
pub const UV_WORK:              uv_req_type = 7;
pub const UV_GETADDRINFO:       uv_req_type = 8;
pub const UV_GETNAMEINFO:       uv_req_type = 9;
pub const UV_REQ_TYPE_PRIVATE:  uv_req_type = 10;
pub const UV_REQ_TYPE_MAX:      uv_req_type = 11;


pub const UV_IGNORE:          uv_stdio_flags = 0x00;
pub const UV_CREATE_PIPE:     uv_stdio_flags = 0x01;
pub const UV_INHERIT_FD:      uv_stdio_flags = 0x02;
pub const UV_INHERIT_STREAM:  uv_stdio_flags = 0x04;
/// When UV_CREATE_PIPE is specified, UV_READABLE_PIPE and UV_WRITABLE_PIPE
/// determine the direction of flow, from the child process' perspective. Both
/// flags may be specified to create a duplex data stream.
pub const UV_READABLE_PIPE:   uv_stdio_flags = 0x10;
pub const UV_WRITABLE_PIPE:   uv_stdio_flags = 0x20;
/// When UV_CREATE_PIPE is specified, specifying UV_NONBLOCK_PIPE opens the
/// handle in non-blocking mode in the child. This may cause loss of data,
/// if the child is not designed to handle to encounter this mode,
/// but can also be significantly more efficient.
pub const UV_NONBLOCK_PIPE:   uv_stdio_flags = 0x40;
pub const UV_OVERLAPPED_PIPE: uv_stdio_flags = 0x40; /* old name, for compatibility */

pub const UV_PROCESS_DEFAULT:                    uv_process_flags  = 0x00;
pub const UV_PROCESS_SETUID:                     uv_process_flags  = 0x01;
pub const UV_PROCESS_SETGID:                     uv_process_flags  = 0x02;
pub const UV_PROCESS_WINDOWS_VERBATIM_ARGUMENTS: uv_process_flags  = 0x04;
pub const UV_PROCESS_DETACHED:                   uv_process_flags  = 0x08;
pub const UV_PROCESS_WINDOWS_HIDE:               uv_process_flags  = 0x10;
pub const UV_PROCESS_WINDOWS_HIDE_CONSOLE:       uv_process_flags  = 0x20;
pub const UV_PROCESS_WINDOWS_HIDE_GUI:           uv_process_flags  = 0x40;

pub type uv_poll_cb = 
  Option<unsafe extern "C" fn(handle: *mut uv_poll_t, status: c_int, evts: c_int)>;

pub type uv__io_cb =
  Option<unsafe extern "C" fn(l: *mut uv_loop_t, w: *mut uv__io_t, events: c_uint)>; 
  
pub type uv_close_cb =  
  Option<unsafe extern "C" fn(handle: *mut uv_handle_t)>;

pub type uv_alloc_cb =
  Option<unsafe extern "C" fn(handle: *mut uv_handle_t, suggested_size: usize, buf: *mut uv_buf_t)>;

pub type uv_walk_cb =
  Option<unsafe extern "C" fn(handle: *mut uv_handle_t, arg: *mut c_void)>;

pub type uv_read_cb =
  Option<unsafe extern "C" fn(stream: *mut uv_stream_t, nread: isize, buf: *const uv_buf_t)>;

pub type uv_write_cb =
  Option<unsafe extern "C" fn(req: *mut uv_write_t, status: c_int)>;

pub type uv_connection_cb =
  Option<unsafe extern "C" fn(server: *mut uv_stream_t, status: c_int)>;

pub type uv_connect_cb =
  Option<unsafe extern "C" fn(req: *mut uv_connect_t, status: c_int)>;

pub type uv_shutdown_cb =
  Option<unsafe extern "C" fn(req: *mut uv_shutdown_t, status: c_int)>;

pub type uv_idle_cb = 
  Option<unsafe extern "C" fn(handle: *mut uv_idle_t)>;

pub type uv_signal_cb = 
  Option<unsafe extern "C" fn(handle: *mut uv_signal_t, signum: c_int)>;

pub type uv_timer_cb =
  Option<unsafe extern "C" fn(handle: *mut uv_timer_t)>;

pub type uv_async_cb =
Option<unsafe extern "C" fn(handle: *mut uv_async_t)>;

pub type uv_prepare_cb =
  Option<unsafe extern "C" fn(handle: *mut uv_prepare_t)>;

pub type uv_check_cb =
  Option<unsafe extern "C" fn(handle: *mut uv_check_t)>;

pub type uv_exit_cb =
  Option<unsafe extern "C" fn(p: *mut uv_process_t, st: i64, sig: c_int)>;

pub type uv_fs_cb =
  Option<unsafe extern "C" fn(req: *mut uv_fs_t)>;

pub type uv_work_cb =
  Option<unsafe extern "C" fn(req: *mut uv_work_t)>;

pub type uv_after_work_cb =
  Option<unsafe extern "C" fn(req: *mut uv_work_t, status: c_int)>;

pub type uv_getaddrinfo_cb =
  Option<unsafe extern "C" fn(req: *mut uv_getaddrinfo_t, status: c_int, res: *mut addrinfo)>;

pub type uv_getnameinfo_cb =
  Option<unsafe extern "C" fn(req: *mut uv_getnameinfo_t, status: c_int, hostname: *const c_char, service: *const c_char)>;

pub type uv_random_cb =
  Option<unsafe extern "C" fn(req: *mut uv_random_t, status: c_int, buf: *mut c_void, buflen: usize)>;

pub type uv_udp_send_cb =
  Option<unsafe extern "C" fn(req: *mut uv_udp_send_t, status: c_int)>;

pub type uv_udp_recv_cb =
  Option<unsafe extern "C" fn(h: *mut uv_udp_t, nread: ssize_t, buf: *const uv_buf_t, addr: *const sockaddr, flags: c_uint)>;




#[repr(C)]
#[derive(Copy, Clone)]
pub union uv_loop_active_reqs_union {
  pub unused: *mut c_void,
  pub count: c_uint,
  pub reserved: [i8; std::mem::size_of::<*mut c_void>()],
  _union_align: [i8; std::mem::size_of::<*mut c_void>()]
}

impl Default for uv_loop_active_reqs_union {
  fn default() -> Self {
    uv_loop_active_reqs_union {
      _union_align:  [0, 0, 0, 0, 0, 0, 0, 0],
    }
  }
}  

impl fmt::Debug for uv_loop_active_reqs_union {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_loop_active_reqs_union")
      // UV_LOOP_PUBLIC_FIELDS
      .field("count", unsafe { &self.count })
      .finish()
  }
}

impl fmt::Display for uv_loop_active_reqs_union {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_loop_timer_heap {
    pub min: *mut c_void,
    pub nelts: c_uint
}

impl Default for uv_loop_timer_heap {
  fn default() -> Self {
    uv_loop_timer_heap {
      min: std::ptr::null_mut(),
      nelts: 0,
    }
  }
}  

#[repr(C)]
pub struct uv_loop_t {
  // UV_LOOP_PUBLIC_FIELDS
  pub data: *mut c_void,
  pub active_handles: c_uint,
  pub handle_queue: [*mut c_void; 2_usize],
  pub active_reqs: uv_loop_active_reqs_union,
  pub internal_fields: *mut c_void,
  pub stop_flag: c_uint,
  // UV_LOOP_PRIVATE_FIELDS
  pub flags: c_ulong,
  pub backend_fd: c_int,
  pub pending_queue: [*mut c_void; 2_usize],
  pub watcher_queue: [*mut c_void; 2_usize],
  pub watchers: *mut *mut uv__io_t,
  pub nwatchers: c_uint,
  pub nfds: c_uint,
  pub wq: [*mut c_void; 2_usize],
  pub wq_mutex: uv_mutex_t,
  pub wq_async: uv_async_t,
  pub cloexec_lock: uv_rwlock_t,
  pub closing_handles: *mut uv_handle_t,
  pub process_handles: [*mut c_void; 2_usize],
  pub prepare_handles: [*mut c_void; 2_usize],
  pub check_handles: [*mut c_void; 2_usize],
  pub idle_handles: [*mut c_void; 2_usize],
  pub async_handles: [*mut c_void; 2_usize],
  pub async_unused: Option<unsafe extern "C" fn()>,
  pub async_io_watcher: uv__io_t,
  pub async_wfd: c_int,
  pub timer_heap: uv_loop_timer_heap,
  pub timer_counter: u64,
  pub time: u64,
  pub signal_pipefd: [c_int; 2_usize],
  pub signal_io_watcher: uv__io_t,
  pub child_watcher: uv_signal_t,
  pub emfile_fd: c_int,
}

impl fmt::Debug for uv_loop_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_loop_t")
      // UV_LOOP_PUBLIC_FIELDS
      .field("data", &self.data)
      .field("active_handles", &self.active_handles)
      .field("handle_queue", &self.handle_queue)
      .field("active_reqs", &self.active_reqs)
      .field("internal_fields", &self.internal_fields)
      .field("stop_flag", &self.stop_flag)
      // UV_LOOP_PRIVATE_FIELDS
      .field("flags", &self.flags)
      .field("backend_fd", &self.backend_fd)
      .field("pending_queue", &self.pending_queue)
      .field("watcher_queue", &self.watcher_queue)
      .field("watchers", &self.watchers)
      .field("nfds", &self.nfds)
      .field("wq", &self.wq)
      // .field("wq_mutex", &self.wq_mutex)
      .field("wq_async", &self.wq_async)
      // .field("cloexec_lock", &self.cloexec_lock)
      .field("closing_handles", &self.closing_handles)
      .field("process_handles", &self.process_handles)
      .field("prepare_handles", &self.prepare_handles)
      .field("check_handles", &self.check_handles)
      .field("idle_handles", &self.idle_handles)
      .field("async_handles", &self.async_handles)
      .field("async_unused", &self.async_unused)
      .field("async_io_watcher", &self.async_io_watcher)
      .field("async_wfd", &self.async_wfd)
      // .field("timer_heap", &self.timer_heap)
      .field("timer_counter", &self.timer_counter)
      .field("time", &self.time)
      .field("signal_pipefd", &self.signal_pipefd)
      .field("signal_io_watcher", &self.signal_io_watcher)
      .field("child_watcher", &self.child_watcher)
      .field("emfile_fd", &self.emfile_fd)
      .finish()
  }
}

impl fmt::Display for uv_loop_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv__io_t {
  pub cb: uv__io_cb,
  pub pending_queue: [*mut c_void; 2_usize],
  pub watcher_queue: [*mut c_void; 2_usize],
  pub pevents: c_int,
  pub events: c_uint,
  pub fd: c_int,
}

impl Default for uv__io_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv__io_t {
      cb:             None,
      pending_queue:  [null, null],
      watcher_queue:  [null, null],
      pevents:        0,
      events:         0,
      fd:             0,     
    }
  }
}
pub trait HandleImpl{}

#[repr(C)]
#[derive(Copy, Clone)]
pub union uv_handle_union {
  pub fd: c_int,
  pub reserved: [*mut c_void; 4_usize],
  _union_align: [u64; 4_usize],
}

impl Default for uv_handle_union {
  fn default() -> Self {
    uv_handle_union {
      _union_align:  [0; 4],
    }
  }
}

impl fmt::Debug for uv_handle_union {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_handle_union")
      .field("fd",  unsafe{ &self.fd })
      .finish()
  }
}

impl fmt::Display for uv_handle_union {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
  
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_handle_t {
  // UV_HANDLE_COMMON_FIELDS
  pub data:         *mut c_void,
  pub loop_:        *mut uv_loop_t,
  pub type_:        uv_handle_type,
  pub close_cb:     uv_close_cb,
  pub handle_queue: [*mut c_void; 2_usize],
  pub u:            uv_handle_union,
  // UV_HANDLE_PRIVATE_FIELDS
  pub next_closing: *mut uv_handle_t,
  pub flags:        c_int,
}

impl Default for uv_handle_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_handle_t {
      data:                   null as *mut c_void,
      loop_:                  null as *mut uv_loop_t,
      type_:                  UV_UNKNOWN_HANDLE,
      close_cb:               None,
      handle_queue:           [null, null],
      u:                      uv_handle_union::default(),
      next_closing:           null as *mut uv_handle_t,
      flags:                  0,
    }
  }
}

impl fmt::Debug for uv_handle_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_handle_t")
      .field("data",  &self.data)
      .field("loop",   &self.loop_)
      .field("type",   &self.type_)
      .field("close_cb",   &self.close_cb)
      .field("handle_queue",   &self.handle_queue)
      .field("u",   &self.u)
      .field("next_closing",   &self.next_closing)
      .field("flags",   &self.flags)
      .finish()
  }
}

impl fmt::Display for uv_handle_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_stream_t {
  // UV_HANDLE_FIELDS
  pub(crate) data:                   *mut c_void,
  pub(crate) loop_:                  *mut uv_loop_t,
  pub(crate) type_:                  uv_handle_type,
  pub(crate) close_cb:               uv_close_cb,
  pub(crate) handle_queue:           [*mut c_void; 2_usize],
  pub(crate) u:                      uv_handle_union,
  pub(crate) next_closing:           *mut uv_handle_t,
  pub(crate) flags:                  c_uint,
  // UV_STREAM_FIELDS
  pub(crate) write_queue_size:       size_t,
  pub(crate) alloc_cb:               uv_alloc_cb,
  pub(crate) read_cb:                uv_read_cb,
  // UV_STREAM_PRIVATE_FIELDS
  pub(crate) connect_req:            *mut uv_connect_t,
  pub(crate) shutdown_req:           *mut uv_shutdown_t,
  pub(crate) io_watcher:             uv__io_t,
  pub(crate) write_queue:            [*mut c_void; 2_usize],
  pub(crate) write_completed_queue:  [*mut c_void; 2_usize],
  pub(crate) connection_cb:          uv_connection_cb,
  pub(crate) delayed_error:          c_int,
  pub(crate) accepted_fd:            c_int,
  pub(crate) queued_fds:             *mut c_void,
}

impl Default for uv_stream_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_stream_t {
      // UV_HANDLE_FIELDS
      data:                   null as *mut c_void,
      loop_:                  null as *mut uv_loop_t,
      type_:                  UV_STREAM,
      close_cb:               None,
      handle_queue:           [null, null],
      u:                      uv_handle_union::default(),
      next_closing:           null as *mut uv_handle_t,
      flags:                  0,
      // UV_STREAM_FIELDS
      write_queue_size:       0,
      alloc_cb:               None,
      read_cb:                None,
      // UV_STREAM_PRIVATE_FIELDS
      connect_req:            null as *mut uv_connect_t,
      shutdown_req:           null as *mut uv_shutdown_t,
      io_watcher:             uv__io_t::default(),
      write_queue:            [null, null],
      write_completed_queue:  [null, null],
      connection_cb:          None,
      delayed_error:          0,
      accepted_fd:            0,
      queued_fds:             null as *mut c_void,        
    }
  }
}

impl fmt::Debug for uv_stream_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_stream_t")
      // UV_HANDLE_FIELDS
      .field("data",  &self.data)
      .field("loop",   &self.loop_)
      .field("type",   &self.type_)
      .field("close_cb",   &self.close_cb)
      .field("handle_queue",   &self.handle_queue)
      .field("u",   &self.u)
      .field("next_closing",   &self.next_closing)
      .field("flags",   &self.flags)
      // UV_STREAM_FIELDS
      .field("write_queue_size",   &self.write_queue_size)
      .field("alloc_cb",   &self.alloc_cb)
      .field("read_cb",   &self.read_cb)
      // UV_STREAM_PRIVATE_FIELDS
      .field("connect_req",   &self.connect_req)
      .field("shutdown_req",   &self.shutdown_req)
      .field("io_watcher",   &self.io_watcher)
      .field("write_queue",   &self.write_queue)
      .field("write_completed_queue",   &self.write_completed_queue)
      .field("connection_cb",   &self.connection_cb)
      .field("delayed_error",   &self.delayed_error)
      .field("accepted_fd",   &self.accepted_fd)
      .field("queued_fds",   &self.queued_fds)
      .finish()
  }
}

impl fmt::Display for uv_stream_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_process_t {
  // UV_HANDLE_FIELDS
  pub data:                   *mut c_void,
  pub loop_:                  *mut uv_loop_t,
  pub type_:                  uv_handle_type,
  pub close_cb:               uv_close_cb,
  pub handle_queue:           [*mut c_void; 2_usize],
  pub u:                      uv_handle_union,
  pub next_closing:           *mut uv_handle_t,
  pub flags:                  c_int,
  // UV_PROCESS_FIELDS
  pub exit_cb:                uv_exit_cb,
  pub pid:                    c_int,
  // UV_PROCESS_PRIVATE_FIELDS
  pub queue:                  [*mut c_void; 2_usize],
  pub status:                 c_int,
}

impl HandleImpl for uv_process_t {}

impl Default for uv_process_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_process_t {
      data:                   null as *mut c_void,
      loop_:                  null as *mut uv_loop_t,
      type_:                  UV_PROCESS,
      close_cb:               None,
      handle_queue:           [null, null],
      u:                      uv_handle_union::default(),
      next_closing:           null as *mut uv_handle_t,
      flags:                  0,
      exit_cb:                None,
      pid:                    0,
      queue:                  [null, null],
      status:                 0
    }
  }
}

impl fmt::Debug for uv_process_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_process_t")
      .field("data",  &self.data)
      .field("loop",   &self.loop_)
      .field("type",   &self.type_)
      .field("close_cb",   &self.close_cb)
      .field("handle_queue",   &self.handle_queue)
      .field("u",   &self.u)
      .field("next_closing",   &self.next_closing)
      .field("flags",   &self.flags)
      .field("exit_cb",   &self.exit_cb)
      .field("pid",   &self.pid)
      .field("queue",   &self.queue)
      .field("status",   &self.status)
      .finish()
  }
}

impl fmt::Display for uv_process_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Deref for uv_process_t {
  type Target = uv_handle_t;

  fn deref(&self) -> &Self::Target {
    unsafe {
      (self as *const uv_process_t as *const uv_handle_t).as_ref().unwrap()
    }
  }
}

impl DerefMut for uv_process_t {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe {
      (self as *mut uv_process_t as *mut uv_handle_t).as_mut().unwrap()
    }
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union uv_stdio_container_union {
  pub stream: *mut uv_stream_t,
  pub fd:     c_int,
  _union_align: [u8; 8_usize],
}

impl Default for uv_stdio_container_union {
  fn default() -> Self {
    uv_stdio_container_union {
      _union_align:  [0; 8],
    }
  }
}

impl fmt::Debug for uv_stdio_container_union {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_stdio_container_union")
      .field("fd",   unsafe{ &self.fd })
      .finish()
  }
}

impl fmt::Display for uv_stdio_container_union {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_stdio_container_t {
  pub flags:  uv_stdio_flags,
  pub data:   uv_stdio_container_union,
}

impl Default for uv_stdio_container_t {
  fn default() -> Self {
    uv_stdio_container_t {
      flags:  UV_IGNORE,
      data:   uv_stdio_container_union::default()
    }
  }
}

impl fmt::Debug for uv_stdio_container_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_stdio_container_t")
      .field("flags",   &self.flags)
      .field("data",   &self.data)
      .finish()
  }
}

impl fmt::Display for uv_stdio_container_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl uv_stdio_container_t {
  pub fn set_readable_fd(&mut self, fd: uv_fd_t) {
    self.flags  = UV_INHERIT_FD;
    self.data   = uv_stdio_container_union { fd };
  }

  pub fn set_readable_stream(&mut self, stream: *mut uv_stream_t) {
    self.flags  = UV_READABLE_PIPE | UV_CREATE_PIPE;
    self.data   = uv_stdio_container_union { stream };
  }

  pub fn set_writable_fd(&mut self, fd: uv_fd_t) {
    self.flags  = UV_INHERIT_FD;
    self.data   = uv_stdio_container_union { fd };
  }

  pub fn set_writable_stream(&mut self, stream: *mut uv_stream_t) {
    self.flags  = UV_WRITABLE_PIPE | UV_CREATE_PIPE;
    self.data   = uv_stdio_container_union { stream };
  }

  pub fn readable_fd(fd: uv_fd_t) -> Self {
    Self {
      flags: UV_INHERIT_FD,
      data:  uv_stdio_container_union { fd }
    }
  }

  pub fn readable_stream(stream: *mut uv_stream_t) -> Self {
    Self {
      flags: UV_READABLE_PIPE | UV_CREATE_PIPE,
      data:  uv_stdio_container_union { stream }
    }
  }

  pub fn writable_fd(fd: uv_fd_t) -> Self {
    Self {
      flags: UV_INHERIT_FD,
      data:  uv_stdio_container_union { fd }
    }
  }

  pub fn writable_stream(stream: *mut uv_stream_t) -> Self {
    Self {
      flags: UV_WRITABLE_PIPE | UV_CREATE_PIPE,
      data:  uv_stdio_container_union { stream }
    }
  }

  pub fn ignore() -> Self {
    Self::default()
  }
}

#[repr(C)]
#[derive(Clone)]
pub struct uv_process_options_t {
  pub exit_cb:      uv_exit_cb,
  pub file:         *const c_char,
  pub args:         *mut *mut c_char,
  // pub env:          *mut *mut c_char,
  pub env:          *mut uv_env_item_t,
  pub cwd:          *const c_char,
  pub flags:        uv_process_flags,
  pub stdio_count:  c_int,
  pub stdio:        *mut uv_stdio_container_t,
  pub uid:          uv_uid_t,
  pub gid:          uv_gid_t,
}

impl uv_process_options_t {
  pub fn args(&self) -> Vec<String> {
    unsafe {
      let mut args: Vec<String> = vec![];
      let mut head = self.args;

      loop {
        let arg = *head;
        if arg.is_null() {
          break;
        }
        args.push( cstr_to_str(arg).to_string() );
        head = head.add(1);
      }

      args
    }
  }

  pub fn env(&self) -> HashMap<String,String> {
    unsafe {
      let mut env = HashMap::default();
      let mut head = self.env;

      loop {
        if head.is_null() {
          break;
        }
        
        let item = *head;

        if item.name.is_null() {
          break;
        }

        let key = cstr_to_str(item.name);
        let val = cstr_to_str(item.value);

        env.insert(key.to_string(), val.to_string());

        head = head.add(1);
      }

      env
    }
  }
}

impl Default for uv_process_options_t {
  fn default() -> Self {
    use std::os::unix::io::AsRawFd;

    let null = std::ptr::null_mut() as *mut c_void;

    uv_process_options_t {
      exit_cb:      None,
      file:         null as *const c_char,
      args:         null as *mut *mut c_char,
      env:          null as *mut uv_env_item_t,
      cwd:          null as *const c_char,
      flags:        UV_PROCESS_DEFAULT,
      stdio_count:  3,
      stdio:        null as *mut uv_stdio_container_t,
      uid:          0,
      gid:          0,
    }
  }
}

impl fmt::Debug for uv_process_options_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    let file = if !self.file.is_null() {
      cstr_to_str(self.file).to_string()
    } else {
      "".to_string()
    };

    let cwd = if !self.cwd.is_null() {
      cstr_to_str(self.cwd).to_string()
    } else {
      "".to_string()
    };

    fmt
      .debug_struct("uv_process_options_t")
      .field("file",  &file)
      .field("args",   &self.args())
      .field("cwd",   &cwd)
      .field("env",   &self.env())
      .field("exit_cb",   &self.exit_cb)
      .finish()
  }
}

impl fmt::Display for uv_process_options_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_poll_t {
  // UV_HANDLE_FIELDS
  pub data:                   *mut c_void,
  pub loop_:                  *mut uv_loop_t,
  pub type_:                  uv_handle_type,
  pub close_cb:               uv_close_cb,
  pub handle_queue:           [*mut c_void; 2_usize],
  pub u:                      uv_handle_union,
  pub next_closing:           *mut uv_handle_t,
  pub flags:                  c_int,
  // UV_POLL_FIELDS
  pub poll_cb:                uv_poll_cb,
  // UV_POLL_PRIVATE_FIELDS
  pub io_watcher:             uv__io_t,
}

impl HandleImpl for uv_poll_t {}

impl Default for uv_poll_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_poll_t {
      data:                   null as *mut c_void,
      loop_:                  null as *mut uv_loop_t,
      type_:                  UV_IDLE,
      close_cb:               None,
      handle_queue:           [null, null],
      u:                      uv_handle_union::default(),
      next_closing:           null as *mut uv_handle_t,
      flags:                  0,
      poll_cb:                None,
      io_watcher:             uv__io_t::default(),
    }
  }
}

impl fmt::Debug for uv_poll_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_poll_t")
      .field("data",  &self.data)
      .field("loop",  &self.loop_)
      .field("type",  &self.type_)
      .field("close_cb",  &self.close_cb)
      .field("handle_queue",  &self.handle_queue)
      .field("next_closing",  &self.next_closing)
      .field("flags",  &self.flags)
      .field("poll_cb",  &self.poll_cb)
      .field("io_watcher",  &self.io_watcher)
      .finish()
  }
}

impl fmt::Display for uv_poll_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_idle_t {
  // UV_HANDLE_FIELDS
  pub data:                   *mut c_void,
  pub loop_:                  *mut uv_loop_t,
  pub type_:                  uv_handle_type,
  pub close_cb:               uv_close_cb,
  pub handle_queue:           [*mut c_void; 2_usize],
  pub u:                      uv_handle_union,
  pub next_closing:           *mut uv_handle_t,
  pub flags:                  c_int,
  // UV_IDLE_PRIVATE_FIELDS
  pub idle_cb:                uv_idle_cb,
  pub queue:                  [*mut c_void; 2_usize],
}

impl HandleImpl for uv_idle_t {}

impl Default for uv_idle_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_idle_t {
      data:                   null as *mut c_void,
      loop_:                  null as *mut uv_loop_t,
      type_:                  UV_IDLE,
      close_cb:               None,
      handle_queue:           [null, null],
      u:                      uv_handle_union::default(),
      next_closing:           null as *mut uv_handle_t,
      flags:                  0,
      idle_cb:                None,
      queue:                  [null, null],
    }
  }
}

impl fmt::Debug for uv_idle_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_idle_t")
      .field("type",  &self.type_)
      .field("loop",  &self.loop_)
      .field("data",  &self.data)
      .field("flags",  &self.flags)
      .field("close_cb",  &self.close_cb)
      .field("idle_cb",  &self.idle_cb)
      .finish()
  }
}

impl fmt::Display for uv_idle_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_async_t {
  // UV_HANDLE_FIELDS
  pub data:                   *mut c_void,
  pub loop_:                  *mut uv_loop_t,
  pub type_:                  uv_handle_type,
  pub close_cb:               uv_close_cb,
  pub handle_queue:           [*mut c_void; 2_usize],
  pub u:                      uv_handle_union,
  pub next_closing:           *mut uv_handle_t,
  pub flags:                  c_int,
  // UV_ASYNC_PRIVATE_FIELDS
  pub async_cb:               uv_async_cb,
  pub queue:                  [*mut c_void; 2_usize],
  pub pending:                c_int,
}

impl HandleImpl for uv_async_t {}

impl Default for uv_async_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_async_t {
      data:                   null as *mut c_void,
      loop_:                  null as *mut uv_loop_t,
      type_:                  UV_ASYNC,
      close_cb:               None,
      handle_queue:           [null, null],
      u:                      uv_handle_union::default(),
      next_closing:           null as *mut uv_handle_t,
      flags:                  0,
      async_cb:                None,
      queue:                  [null, null],
      pending:                0,
    }
  }
}

impl fmt::Debug for uv_async_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_idle_t")
      .field("type",  &self.type_)
      .field("loop",  &self.loop_)
      .field("data",  &self.data)
      .field("flags",  &self.flags)
      .field("close_cb",  &self.close_cb)
      .field("async_cb",  &self.async_cb)
      .field("pending",  &self.pending)
      .finish()
  }
}

impl fmt::Display for uv_async_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_req_t {
  // UV_REQ_COMMON_FIELDS
  pub data:         *mut c_void,
  pub type_:        uv_req_type,
  pub reserved:     [*mut c_void; 6],
  // UV_REQ_PRIVATE_FIELDS
}

impl Default for uv_req_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_req_t {
      data:     null,
      type_:    UV_UNKNOWN_REQ,
      reserved: [null, null, null, null, null, null],
    }
  }
}

impl fmt::Debug for uv_req_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_req_t")
      // UV_HANDLE_FIELDS
      .field("data",  &self.data)
      .field("type",  &self.type_)
      .field("reserved",  &self.reserved)
      .finish()
  }
}

impl fmt::Display for uv_req_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_write_t {
  // UV_REQ_COMMON_FIELDS
  pub data:         *mut c_void,
  pub type_:        uv_req_type,
  pub reserved:     [*mut c_void; 6],
  // UV_WRITE_FIELDS
  pub cb:           uv_write_cb,
  pub send_handle:  *mut uv_stream_t,
  pub handle:       *mut uv_stream_t,
  // UV_WRITE_PRIVATE_FIELDS
  pub queue:        [*mut c_void; 2],
  pub write_index:  c_uint,
  pub bufs:         *mut uv_buf_t,
  pub nbufs:        c_uint,
  pub error:        c_int,
  pub bufsml:       [uv_buf_t; 4],
}

impl Default for uv_write_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_write_t {
      // UV_REQ_COMMON_FIELDS
      data:         null,
      type_:        UV_WRITE,
      reserved:     [null; 6],
      // UV_WRITE_FIELDS
      cb:           None,
      send_handle:  null as *mut uv_stream_t,
      handle:       null as *mut uv_stream_t,
      // UV_WRITE_PRIVATE_FIELDS
      queue:        [null; 2],
      write_index:  0,
      bufs:         null as *mut uv_buf_t,
      nbufs:        0,
      error:        0,
      bufsml:       [uv_buf_t::default(); 4],
    }
  }
}



#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_connect_t {
  // UV_REQ_COMMON_FIELDS
  pub data:         *mut c_void,
  pub type_:        uv_req_type,
  pub reserved:     [*mut c_void; 6],
  // UV_CONNECT_FIELDS
  pub cb:           uv_connect_cb,
  pub handle:       *mut uv_stream_t,
  // UV_CONNECT_PRIVATE_FIELDS
  pub queue:        [*mut c_void; 2_usize],
}

impl Default for uv_connect_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_connect_t {
      data:     null,
      type_:    UV_UNKNOWN_REQ,
      handle:   null as *mut uv_stream_t,
      reserved: [null, null, null, null, null, null],
      cb:       None,
      queue:    [null, null],
    }
  }
}

impl fmt::Debug for uv_connect_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_connect_t")
      // UV_REQ_COMMON_FIELDS
      .field("data",  &self.data)
      .field("type",  &self.type_)
      .field("reserved",  &self.reserved)
      // UV_CONNECT_FIELDS
      .field("cb",  &self.cb)
      .field("handle",  &self.handle)
      // UV_CONNECT_PRIVATE_FIELDS
      .field("queue",  &self.queue)
      .finish()
  }
}

impl fmt::Display for uv_connect_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_shutdown_t {
  // UV_REQ_COMMON_FIELDS
  pub data:         *mut c_void,
  pub type_:        uv_req_type,
  pub reserved:     [*mut c_void; 6],
  // UV_SHUTDOWN_FIELDS
  pub handle:       *mut uv_stream_t,
  pub cb:           uv_shutdown_cb,
  // UV_SHUTDOWN_PRIVATE_FIELDS
}

impl Default for uv_shutdown_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_shutdown_t {
      data:     null,
      type_:    UV_UNKNOWN_REQ,
      reserved: [null, null, null, null, null, null],
      handle:   null as *mut uv_stream_t,
      cb:       None,
    }
  }
}

impl fmt::Debug for uv_shutdown_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_shutdown_t")
      // UV_HANDLE_FIELDS
      .field("data",  &self.data)
      .field("type",  &self.type_)
      .field("reserved",  &self.reserved)
      // UV_SHUTDOWN_FIELDS
      .field("handle",  &self.handle)
      .field("cb",  &self.cb)
      .finish()
  }
}

impl fmt::Display for uv_shutdown_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_pipe_t {
  // UV_HANDLE_FIELDS
  pub data:                   *mut c_void,
  pub loop_:                  *mut uv_loop_t,
  pub type_:                  uv_handle_type,
  pub close_cb:               uv_close_cb,
  pub handle_queue:           [*mut c_void; 2_usize],
  pub u:                      uv_handle_union,
  pub next_closing:           *mut uv_handle_t,
  pub flags:                  c_int,
  // UV_STREAM_FIELDS
  pub write_queue_size:       size_t,
  pub alloc_cb:               uv_alloc_cb,
  pub read_cb:                uv_read_cb,
  pub connect_req:            *mut uv_connect_t,
  pub shutdown_req:           *mut uv_shutdown_t,
  pub io_watcher:             uv__io_t,
  pub write_queue:            [*mut c_void; 2_usize],
  pub write_completed_queue:  [*mut c_void; 2_usize],
  pub connection_cb:          uv_connection_cb,
  pub delayed_error:          c_int,
  pub accepted_fd:            c_int,
  pub queued_fds:             *mut c_void,
  // UV_PIPE_FIELDS
  pub ipc:                    c_int,
  // UV_PIPE_PRIVATE_FIELDS
  pub pipe_fname:             *const c_char,
}

impl HandleImpl for uv_pipe_t {}

impl Default for uv_pipe_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_pipe_t {
      // UV_HANDLE_FIELDS
      data:                   null as *mut c_void,
      loop_:                  null as *mut uv_loop_t,
      type_:                  UV_NAMED_PIPE,
      close_cb:               None,
      handle_queue:           [null, null],
      u:                      uv_handle_union::default(),
      next_closing:           null as *mut uv_handle_t,
      flags:                  0,
      // UV_STREAM_FIELDS
      write_queue_size:       0,
      alloc_cb:               None,
      read_cb:                None,
      connect_req:            null as *mut uv_connect_t,
      shutdown_req:           null as *mut uv_shutdown_t,
      io_watcher:             uv__io_t::default(),
      write_queue:            [null, null],
      write_completed_queue:  [null, null],
      connection_cb:          None,
      delayed_error:          0,
      accepted_fd:            0,
      queued_fds:             null as *mut c_void,
      // UV_PIPE_FIELDS
      ipc:                    0,
      // UV_PIPE_PRIVATE_FIELDS
      pipe_fname:             null as *const c_char,
    }
  }
}

impl fmt::Debug for uv_pipe_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_pipe_t")
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_tcp_t {
  // UV_HANDLE_FIELDS
  pub data:                   *mut c_void,
  pub loop_:                  *mut uv_loop_t,
  pub type_:                  uv_handle_type,
  pub close_cb:               uv_close_cb,
  pub handle_queue:           [*mut c_void; 2_usize],
  pub u:                      uv_handle_union,
  pub next_closing:           *mut uv_handle_t,
  pub flags:                  c_int,
  // UV_STREAM_FIELDS
  pub write_queue_size:       size_t,
  pub alloc_cb:               uv_alloc_cb,
  pub read_cb:                uv_read_cb,
  pub connect_req:            *mut uv_connect_t,
  pub shutdown_req:           *mut uv_shutdown_t,
  pub io_watcher:             uv__io_t,
  pub write_queue:            [*mut c_void; 2_usize],
  pub write_completed_queue:  [*mut c_void; 2_usize],
  pub connection_cb:          uv_connection_cb,
  pub delayed_error:          c_int,
  pub accepted_fd:            c_int,
  pub queued_fds:             *mut c_void,
  // UV_TCP_PRIVATE_FIELDS
}

impl HandleImpl for uv_tcp_t {}

impl Default for uv_tcp_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_tcp_t {
      // UV_HANDLE_FIELDS
      data:                   null as *mut c_void,
      loop_:                  null as *mut uv_loop_t,
      type_:                  UV_NAMED_PIPE,
      close_cb:               None,
      handle_queue:           [null, null],
      u:                      uv_handle_union::default(),
      next_closing:           null as *mut uv_handle_t,
      flags:                  0,
      // UV_STREAM_FIELDS
      write_queue_size:       0,
      alloc_cb:               None,
      read_cb:                None,
      connect_req:            null as *mut uv_connect_t,
      shutdown_req:           null as *mut uv_shutdown_t,
      io_watcher:             uv__io_t::default(),
      write_queue:            [null, null],
      write_completed_queue:  [null, null],
      connection_cb:          None,
      delayed_error:          0,
      accepted_fd:            0,
      queued_fds:             null as *mut c_void,
      // UV_TCP_PRIVATE_FIELDS
    }
  }
}

impl fmt::Debug for uv_tcp_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_tcp_t")
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
      // UV_TCP_PRIVATE_FIELDS
      .finish()
  }
}

impl fmt::Display for uv_tcp_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_udp_t {
  // UV_HANDLE_FIELDS ---------------------------------------------------------
  pub data:                   *mut c_void,
  pub loop_:                  *mut uv_loop_t,
  pub type_:                  uv_handle_type,
  pub close_cb:               uv_close_cb,
  pub handle_queue:           [*mut c_void; 2_usize],
  pub u:                      uv_handle_union,
  pub next_closing:           *mut uv_handle_t,
  pub flags:                  c_int,
  // UV_UDP_FIELDS ------------------------------------------------------------
  // Number of bytes queued for sending (read-only).
  // This field strictly shows how much information is currently queued.
  pub send_queue_size:        size_t,
  // Number of send requests currently in the queue awaiting to be processed.
  pub send_queue_count:       size_t,
  // UV_UDP_PRIVATE_FIELDS ----------------------------------------------------
  pub alloc_cb:               uv_alloc_cb,
  pub recv_cb:                uv_udp_recv_cb,
  pub io_watcher:             uv__io_t,
  pub write_queue:            [*mut c_void; 2_usize],
  pub write_completed_queue:  [*mut c_void; 2_usize],
}

impl HandleImpl for uv_udp_t {}

impl Default for uv_udp_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_udp_t {
      // UV_HANDLE_FIELDS
      data:                   null as *mut c_void,
      loop_:                  null as *mut uv_loop_t,
      type_:                  UV_NAMED_PIPE,
      close_cb:               None,
      handle_queue:           [null, null],
      u:                      uv_handle_union::default(),
      next_closing:           null as *mut uv_handle_t,
      flags:                  0,
      // UV_UDP_FIELDS
      send_queue_size:        0,
      send_queue_count:       0,
      // UV_UDP_PRIVATE_FIELDS
      alloc_cb:               None,
      recv_cb:                None,
      io_watcher:             uv__io_t::default(),
      write_queue:            [null, null],
      write_completed_queue:  [null, null],
    }
  }
}

impl fmt::Debug for uv_udp_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_udp_t")
      // UV_HANDLE_FIELDS
      .field("data",  &self.data)
      .field("loop",  &self.loop_)
      .field("type",  &self.type_)
      .field("close_cb",  &self.close_cb)
      .field("handle_queue",  &self.handle_queue)
      .field("next_closing",  &self.next_closing)
      .field("flags",  &self.flags)
      // UV_UDP_FIELDS
      .field("send_queue_size",  &self.send_queue_size)
      .field("send_queue_count",  &self.send_queue_count)
      // UV_UDP_PRIVATE_FIELDS
      .field("alloc_cb",  &self.alloc_cb)
      .field("recv_cb",  &self.recv_cb)
      .field("io_watcher",  &self.io_watcher)
      .field("write_queue",  &self.write_queue)
      .field("write_completed_queue",  &self.write_completed_queue)
      // UV_TCP_PRIVATE_FIELDS
      .finish()
  }
}

impl fmt::Display for uv_udp_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct termios_t {
  pub c_iflag:  tcflag_t,
  pub c_oflag:  tcflag_t,
  pub c_cflag:  tcflag_t,
  pub c_lflag:  tcflag_t,
  pub c_cc:     [cc_t; NCCS],
  pub c_ispeed: speed_t,
  pub c_ospeed: speed_t,  
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_tty_t {
  // UV_HANDLE_FIELDS
  pub data:                   *mut c_void,
  pub loop_:                  *mut uv_loop_t,
  pub type_:                  uv_handle_type,
  pub close_cb:               uv_close_cb,
  pub handle_queue:           [*mut c_void; 2_usize],
  pub u:                      uv_handle_union,
  pub next_closing:           *mut uv_handle_t,
  pub flags:                  c_int,
  // UV_STREAM_FIELDS
  pub write_queue_size:       size_t,
  pub alloc_cb:               uv_alloc_cb,
  pub read_cb:                uv_read_cb,
  pub connect_req:            *mut uv_connect_t,
  pub shutdown_req:           *mut uv_shutdown_t,
  pub io_watcher:             uv__io_t,
  pub write_queue:            [*mut c_void; 2_usize],
  pub write_completed_queue:  [*mut c_void; 2_usize],
  pub connection_cb:          uv_connection_cb,
  pub delayed_error:          c_int,
  pub accepted_fd:            c_int,
  pub queued_fds:             *mut c_void,
  // UV_TTY_PRIVATE_FIELDS
  pub orig_termios:           termios,
  pub mode:                   c_int,
}

impl HandleImpl for uv_tty_t {}

impl Default for uv_tty_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_tty_t {
      // UV_HANDLE_FIELDS
      data:                   null as *mut c_void,
      loop_:                  null as *mut uv_loop_t,
      type_:                  UV_NAMED_PIPE,
      close_cb:               None,
      handle_queue:           [null, null],
      u:                      uv_handle_union::default(),
      next_closing:           null as *mut uv_handle_t,
      flags:                  0,
      // UV_STREAM_FIELDS
      write_queue_size:       0,
      alloc_cb:               None,
      read_cb:                None,
      connect_req:            null as *mut uv_connect_t,
      shutdown_req:           null as *mut uv_shutdown_t,
      io_watcher:             uv__io_t::default(),
      write_queue:            [null, null],
      write_completed_queue:  [null, null],
      connection_cb:          None,
      delayed_error:          0,
      accepted_fd:            0,
      queued_fds:             null as *mut c_void,
      // UV_TCP_PRIVATE_FIELDS
      orig_termios: termios {
        c_iflag:  0,
        c_oflag:  0,
        c_cflag:  0,
        c_lflag:  0,
        c_cc:     [0; NCCS],
        c_ispeed: 0,
        c_ospeed: 0,      
      },
      mode: 0,
    }
  }
}

impl fmt::Debug for uv_tty_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_tty_t")
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
      // UV_TCP_PRIVATE_FIELDS
      .finish()
  }
}

impl fmt::Display for uv_tty_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_signal_tree_entry {
  pub rbe_left:   *mut uv_signal_t,
  pub rbe_right:  *mut uv_signal_t,
  pub rbe_parent: *mut uv_signal_t,
  pub rbe_color:  c_int
}

impl Default for uv_signal_tree_entry {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut uv_signal_t;

    uv_signal_tree_entry {
      rbe_left:   null,
      rbe_right:  null,
      rbe_parent: null,
      rbe_color:  0,
    }
  }
}

impl fmt::Debug for uv_signal_tree_entry {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_signal_tree_entry")
      .field("rbe_left",  &self.rbe_left)
      .field("rbe_right",  &self.rbe_right)
      .field("rbe_parent",  &self.rbe_parent)
      .field("rbe_color",  &self.rbe_color)
      .finish()
  }
}

impl fmt::Display for uv_signal_tree_entry {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_signal_t {
  // UV_HANDLE_FIELDS
  pub data:                   *mut c_void,
  pub loop_:                  *mut uv_loop_t,
  pub type_:                  uv_handle_type,
  pub close_cb:               uv_close_cb,
  pub handle_queue:           [*mut c_void; 2_usize],
  pub u:                      uv_handle_union,
  pub next_closing:           *mut uv_handle_t,
  pub flags:                  c_int,
  // UV_SIGNAL_FIELDS
  pub signal_cb:              uv_signal_cb,
  pub signum:                 c_int,
  // UV_SIGNAL_PRIVATE_FIELDS
  pub tree_entry:             uv_signal_tree_entry,
  pub caught_signals:         c_uint,
  pub dispatched_signals:     c_uint,
}

impl HandleImpl for uv_signal_t {}

impl Default for uv_signal_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_signal_t {
      // UV_HANDLE_FIELDS
      data:                   null as *mut c_void,
      loop_:                  null as *mut uv_loop_t,
      type_:                  UV_SIGNAL,
      close_cb:               None,
      handle_queue:           [null, null],
      u:                      uv_handle_union::default(),
      next_closing:           null as *mut uv_handle_t,
      flags:                  0,
      // UV_SIGNAL_FIELDS
      signal_cb:              None,
      signum:                 0,
      // UV_SIGNAL_PRIVATE_FIELDS
      tree_entry:             uv_signal_tree_entry::default(),
      caught_signals:         0,
      dispatched_signals:     0,
    }
  }
}

impl fmt::Debug for uv_signal_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_signal_t")
      .field("data",  &self.data)
      .field("loop",  &self.loop_)
      .field("type",  &self.type_)
      .field("close_cb",  &self.close_cb)
      .field("handle_queue",  &self.handle_queue)
      .field("u",  &self.u)
      .field("next_closing",  &self.next_closing)
      .field("flags",  &self.flags)
      .field("signal_cb",  &self.signal_cb)
      .field("signum",  &self.signum)
      .field("tree_entry",  &self.tree_entry)
      .field("caught_signals",  &self.caught_signals)
      .field("dispatched_signals",  &self.dispatched_signals)
      .finish()
  }
}

impl fmt::Display for uv_signal_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub struct uv_timer_t {
  // UV_HANDLE_FIELDS
  pub data:                   *mut c_void,
  pub loop_:                  *mut uv_loop_t,
  pub type_:                  uv_handle_type,
  pub close_cb:               uv_close_cb,
  pub handle_queue:           [*mut c_void; 2_usize],
  pub u:                      uv_handle_union,
  pub next_closing:           *mut uv_handle_t,
  pub flags:                  c_int,
  // UV_TIMER_PRIVATE_FIELDS
  pub timer_cb:               uv_signal_cb,
  pub heap_node:              [*mut c_void; 3_usize],
  pub timeout:                u64,
  pub repeat:                 u64,
  pub start_id:               u64,
}

impl HandleImpl for uv_timer_t {}

impl Default for uv_timer_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_timer_t {
      data:                   null as *mut c_void,
      loop_:                  null as *mut uv_loop_t,
      type_:                  UV_SIGNAL,
      close_cb:               None,
      handle_queue: [null, null],
      u:            uv_handle_union::default(),
      next_closing: null as *mut uv_handle_t,
      flags:        0,
      timer_cb:     None,
      heap_node:    [null, null, null],
      timeout:      0,
      repeat:       0,
      start_id:     0,
    }
  }
}

impl fmt::Debug for uv_timer_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_timer_t")
      .field("data",  &self.data)
      .field("loop",  &self.loop_)
      .field("type",  &self.type_)
      .field("close_cb",  &self.close_cb)
      .field("handle_queue",  &self.handle_queue)
      .field("next_closing",  &self.next_closing)
      .field("flags",  &self.flags)
      .field("timer_cb",  &self.timer_cb)
      .field("heap_node",  &self.heap_node)
      .field("timeout",  &self.timeout)
      .field("repeat",  &self.repeat)
      .field("start_id",  &self.start_id)
      .finish()
  }
}

impl fmt::Display for uv_timer_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_check_t {
  // UV_HANDLE_FIELDS
  pub data:                   *mut c_void,
  pub loop_:                  *mut uv_loop_t,
  pub type_:                  uv_handle_type,
  pub close_cb:               uv_close_cb,
  pub handle_queue:           [*mut c_void; 2_usize],
  pub u:                      uv_handle_union,
  pub next_closing:           *mut uv_handle_t,
  pub flags:                  c_int,
  // UV_CHECK_PRIVATE_FIELDS
  pub check_cb:                uv_check_cb,
  pub queue:                  [*mut c_void; 2_usize],
}

impl HandleImpl for uv_check_t {}

impl Default for uv_check_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_void;

    uv_check_t {
      data:                   null as *mut c_void,
      loop_:                  null as *mut uv_loop_t,
      type_:                  UV_CHECK,
      close_cb:               None,
      handle_queue:           [null, null],
      u:                      uv_handle_union::default(),
      next_closing:           null as *mut uv_handle_t,
      flags:                  0,
      check_cb:               None,
      queue:                  [null, null],
    }
  }
}

impl fmt::Debug for uv_check_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("uv_check_t")
      .field("data", &self.data)
      .field("loop", &self.loop_)
      .field("type", &self.type_)
      .field("close_cb",  &self.close_cb)
      .field("handle_queue", &self.handle_queue)
      .field("next_closing", &self.next_closing)
      .field("flags", &self.flags)
      .field("check_cb", &self.check_cb)
      .field("queue",  &self.queue)
      .finish()
  }
}

impl fmt::Display for uv_check_t {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}



#[repr(C)]
pub struct addrinfo                { _private: [u8; 0] }


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr {
  pub sa_family: sa_family_t,
  pub sa_data: [c_char; 14_usize],
}

impl Default for sockaddr {
  fn default() -> Self {
    sockaddr {
      sa_family:  0,
      sa_data:    [0; 14],
    }
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_timespec_t {
  pub tv_sec:   c_long,
  pub tv_nsec:  c_long,
}

impl Default for uv_timespec_t {
  fn default() -> Self {
    uv_timespec_t {
      tv_sec:   0,
      tv_nsec:  0,
    }
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_timeval_t {
  pub tv_sec:   c_long,
  pub tv_usec:  c_long,
}

impl Default for uv_timeval_t {
  fn default() -> Self {
    uv_timeval_t {
      tv_sec:   0,
      tv_usec:  0,
    }
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_timeval64_t {
  pub tv_sec:   i64,
  pub tv_usec:  c_int,
}

impl Default for uv_timeval64_t {
  fn default() -> Self {
    uv_timeval64_t {
      tv_sec:   0,
      tv_usec:  0,
    }
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_rusage_t {
  /// User CPU time used
  pub ru_utime: uv_timeval_t,
  /// System CPU time used
  pub ru_stime: uv_timeval_t,
  /// Maximum resident set size
  pub ru_maxrss: u64,
  /// Integral shared memory size
  pub ru_ixrss: u64,
  /// Integral unshared data size
  pub ru_idrss: u64,
  /// Integral unshared stack size
  pub ru_isrss: u64,
  /// Page reclaims (soft page faults)
  pub ru_minflt: u64,
  /// Page faults (hard page faults)
  pub ru_majflt: u64,
  /// Swaps
  pub ru_nswap: u64,
  /// Block input operations
  pub ru_inblock: u64,
  /// Block output operations
  pub ru_oublock: u64,
  /// IPC messages sent
  pub ru_msgsnd: u64,
  /// IPC messages received
  pub ru_msgrcv: u64,
  /// Signals received
  pub ru_nsignals: u64,
  /// Voluntary context switches
  pub ru_nvcsw: u64,
  /// Involuntary context switches
  pub ru_nivcsw:u64,
}

impl Default for uv_rusage_t {
  fn default() -> Self {
    uv_rusage_t {
      ru_utime:     uv_timeval_t::default(),
      ru_stime:     uv_timeval_t::default(),
      ru_maxrss:    0,
      ru_ixrss:     0,
      ru_idrss:     0,
      ru_isrss:     0,
      ru_minflt:    0,
      ru_majflt:    0,
      ru_nswap:     0,
      ru_inblock:   0,
      ru_oublock:   0,
      ru_msgsnd:    0,
      ru_msgrcv:    0,
      ru_nsignals:  0,
      ru_nvcsw:     0,
      ru_nivcsw:    0,
    }
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_stat_t {
  pub st_dev:       u64,
  pub st_mode:      u64,
  pub st_nlink:     u64,
  pub st_uid:       u64,
  pub st_gid:       u64,
  pub st_rdev:      u64,
  pub st_ino:       u64,
  pub st_size:      u64,
  pub st_blksize:   u64,
  pub st_blocks:    u64,
  pub st_flags:     u64,
  pub st_gen:       u64,
  pub st_atim:      uv_timespec_t,
  pub st_mtim:      uv_timespec_t,
  pub st_ctim:      uv_timespec_t,
  pub st_birthtim:  uv_timespec_t,
}

impl Default for uv_stat_t {
  fn default() -> Self {
    uv_stat_t {
      st_dev:       0,
      st_mode:      0,
      st_nlink:     0,
      st_uid:       0,
      st_gid:       0,
      st_rdev:      0,
      st_ino:       0,
      st_size:      0,
      st_blksize:   0,
      st_blocks:    0,
      st_flags:     0,
      st_gen:       0,
      st_atim:      uv_timespec_t::default(),
      st_mtim:      uv_timespec_t::default(),
      st_ctim:      uv_timespec_t::default(),
      st_birthtim:  uv_timespec_t::default(),
    }
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_cpu_times_t {
  /// milliseconds
  pub user: u64,
  /// milliseconds
  pub nice: u64,
  /// milliseconds
  pub sys: u64,
  /// milliseconds
  pub idle: u64,
  /// milliseconds
  pub irq: u64,
}

impl Default for uv_cpu_times_t {
  fn default() -> Self {
    uv_cpu_times_t {
      user: 0,
      nice: 0,
      sys: 0,
      idle: 0,
      irq: 0,
    }
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_cpu_info_t {
  pub model: *mut c_char,
  pub speed: c_int,
  pub cpu_times: uv_cpu_times_t,
}

impl Default for uv_cpu_info_t {
  fn default() -> Self {
    uv_cpu_info_t {
      model: std::ptr::null_mut(),
      speed: 0,
      cpu_times: uv_cpu_times_t::default(),
    }
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_buf_t {
  pub base: *mut c_char,
  pub len: usize,
}

impl Default for uv_buf_t {
  fn default() -> Self {
    uv_buf_t {
      base: std::ptr::null_mut(),
      len: 0,
    }
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_passwd_t {
  pub username: *mut c_char,
  pub uid: c_long,
  pub gid: c_long,
  pub shell: *mut c_char,
  pub homedir: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_utsname_t {
  pub sysname: [c_char; 256_usize],
  pub release: [c_char; 256_usize],
  pub version: [c_char; 256_usize],
  pub machine: [c_char; 256_usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_env_item_t {
  pub name:   *mut c_char,
  pub value:  *mut c_char,
}

impl uv_env_item_t {
  pub fn set(&mut self, name: *mut c_char, value: *mut c_char) {
    self.name = name;
    self.value = value;
  }

  pub fn set_name(&mut self, name: *mut c_char) {
    self.name = name;
  }

  pub fn set_value(&mut self, value: *mut c_char) {
    self.value = value;
  }
}

impl Default for uv_env_item_t {
  fn default() -> Self {
    let null = std::ptr::null_mut() as *mut c_char;

    uv_env_item_t {
      name: null,
      value: null,
    }
  }
}

impl fmt::Debug for uv_env_item_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    let key = if self.name.is_null() {
      "".to_string()
    } else {
      cstr_to_str(self.name).to_string()
    };

    let val = if self.value.is_null() {
      "".to_string()
    } else {
      cstr_to_str(self.value).to_string()
    };    

    fmt
      .debug_struct("uv_env_item_t")
      .field("key",  &key)
      .field("val",  &val)
      .finish()
  }
}

impl fmt::Display for uv_env_item_t {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    write!(fmt, "{:?}", self)
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv__work {
  pub work: Option<unsafe extern "C" fn(w: *mut uv__work)>,
  pub done: Option<unsafe extern "C" fn(w: *mut uv__work, status: c_int)>,
  pub loop_: *mut uv_loop_t,
  pub wq: [*mut c_void; 2_usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_random_t {
  pub data: *mut c_void,
  pub type_: uv_req_type,
  pub reserved: [*mut c_void; 6usize],
  pub loop_: *mut uv_loop_t,
  pub status: c_int,
  pub buf: *mut c_void,
  pub buflen: usize,
  pub cb: uv_random_cb,
  pub work_req: uv__work,
}


#[repr(C)] pub struct multicast_addr          { _private: [u8; 0] }
#[repr(C)] pub struct interface_addr          { _private: [u8; 0] }
#[repr(C)] pub struct source_addr             { _private: [u8; 0] }


#[repr(C)] pub struct uv_dir_t                { _private: [u8; 0] }

#[repr(C)] pub struct uv_prepare_t            { _private: [u8; 0] }

#[repr(C)] pub struct uv_fs_event_t           { _private: [u8; 0] }
#[repr(C)] pub struct uv_fs_poll_t            { _private: [u8; 0] }

/// Req types
#[repr(C)] pub struct uv_getaddrinfo_t        { _private: [u8; 0] }
#[repr(C)] pub struct uv_getnameinfo_t        { _private: [u8; 0] }

#[repr(C)] pub struct uv_read_t               { _private: [u8; 0] }

#[repr(C)] pub struct uv_udp_send_t           { _private: [u8; 0] }
#[repr(C)] pub struct uv_fs_t                 { _private: [u8; 0] }
#[repr(C)] pub struct uv_work_t               { _private: [u8; 0] }
#[repr(C)] pub struct uv_interface_address_t  { _private: [u8; 0] }
#[repr(C)] pub struct uv_dirent_t             { _private: [u8; 0] }
#[repr(C)] pub struct uv_statfs_t             { _private: [u8; 0] }



extern "C" {
  pub fn uv_handle_size(type_: uv_handle_type) -> usize;
  pub fn uv_handle_get_type(handle: *const uv_handle_t) -> uv_handle_type;
  pub fn uv_handle_type_name(type_: uv_handle_type) -> *const c_char;
  pub fn uv_handle_get_data(handle: *const uv_handle_t) -> *mut c_void;
  pub fn uv_handle_get_loop(handle: *const uv_handle_t) -> *mut uv_loop_t;
  pub fn uv_handle_set_data(handle: *mut uv_handle_t, data: *mut c_void);

  pub fn uv_is_closing(h: *const uv_handle_t) -> c_int;    
  pub fn uv_is_active(handle: *const uv_handle_t) -> c_int;
  pub fn uv_close(handle: *mut uv_handle_t, close_cb: uv_close_cb);
  pub fn uv_fileno(handle: *const uv_handle_t, fd: *mut uv_os_fd_t) -> c_int;
  pub fn uv_walk(loop_: *mut uv_loop_t, walk_cb: uv_walk_cb, arg: *mut c_void);    
  pub fn uv_send_buffer_size(handle: *mut uv_handle_t, value: *mut c_int) -> c_int;
  pub fn uv_recv_buffer_size(handle: *mut uv_handle_t, value: *mut c_int) -> c_int;

  pub(crate) fn uv_idle_init(l: *mut uv_loop_t, h: *mut uv_idle_t) -> c_int;
  pub(crate) fn uv_idle_start(h: *mut uv_idle_t, cb: uv_idle_cb) -> c_int;
  pub(crate) fn uv_idle_stop(h: *mut uv_idle_t) -> c_int;  

  pub fn uv_stream_get_write_queue_size(stream: *const uv_stream_t) -> size_t;

  pub fn uv_listen(stream: *mut uv_stream_t, backlog: c_int, cb: uv_connection_cb) -> c_int;
  pub fn uv_accept(server: *mut uv_stream_t, client: *mut uv_stream_t) -> c_int;

  pub fn uv_read_start(arg1: *mut uv_stream_t, alloc_cb: uv_alloc_cb, read_cb: uv_read_cb) -> c_int;
  pub fn uv_read_stop(arg1: *mut uv_stream_t) -> c_int;

  pub fn uv_write(r: *mut uv_write_t, h: *mut uv_stream_t, b: *const uv_buf_t, n: c_uint, cb: uv_write_cb) -> c_int;
  pub fn uv_write2(r: *mut uv_write_t, h: *mut uv_stream_t, b: *const uv_buf_t, n: c_uint, s: *mut uv_stream_t, cb: uv_write_cb) -> c_int;
  pub fn uv_try_write(handle: *mut uv_stream_t, bufs: *const uv_buf_t, nbufs: c_uint) -> c_int;

  pub fn uv_is_readable(handle: *const uv_stream_t) -> c_int;
  pub fn uv_is_writable(handle: *const uv_stream_t) -> c_int;

  pub fn uv_stream_set_blocking(handle: *mut uv_stream_t, blocking: c_int) -> c_int;


  pub fn uv_shutdown(req: *mut uv_shutdown_t, handle: *mut uv_stream_t, cb: uv_shutdown_cb) -> c_int;  

  pub fn uv_poll_init(l: *mut uv_loop_t, h: *mut uv_poll_t, fd: c_int) -> c_int;
  pub fn uv_poll_init_socket(l: *mut uv_loop_t, h: *mut uv_poll_t, sock: uv_os_sock_t) -> c_int;
  pub fn uv_poll_start(h: *mut uv_poll_t, evts: c_int, cb: uv_poll_cb) -> c_int;
  pub fn uv_poll_stop(h: *mut uv_poll_t) -> c_int;

  pub(crate) fn uv_check_init(l: *mut uv_loop_t, h: *mut uv_check_t) -> c_int;
  pub(crate) fn uv_check_start(h: *mut uv_check_t, cb: uv_check_cb) -> c_int;
  pub(crate) fn uv_check_stop(h: *mut uv_check_t) -> c_int;

  pub fn uv_tcp_init(l: *mut uv_loop_t, handle: *mut uv_tcp_t) -> c_int;
  pub fn uv_tcp_init_ex(l: *mut uv_loop_t, h: *mut uv_tcp_t, flags: c_uint) -> c_int;
  pub fn uv_tcp_open(h: *mut uv_tcp_t, sock: uv_os_sock_t) -> c_int;
  pub fn uv_tcp_nodelay(h: *mut uv_tcp_t, enable: c_int) -> c_int;
  pub fn uv_tcp_keepalive(h: *mut uv_tcp_t, enable: c_int, delay: c_uint) -> c_int;
  pub fn uv_tcp_simultaneous_accepts(h: *mut uv_tcp_t, enable: c_int) -> c_int;
  pub fn uv_tcp_bind(h: *mut uv_tcp_t, addr: *const sockaddr, flags: c_uint) -> c_int;
  pub fn uv_tcp_getsockname(h: *const uv_tcp_t, name: *mut sockaddr, namelen: *mut c_int) -> c_int;
  pub fn uv_tcp_getpeername(h: *const uv_tcp_t, name: *mut sockaddr, namelen: *mut c_int) -> c_int;
  pub fn uv_tcp_close_reset(h: *mut uv_tcp_t, close_cb: uv_close_cb) -> c_int;
  pub fn uv_tcp_connect(r: *mut uv_connect_t, h: *mut uv_tcp_t, addr: *const sockaddr, cb: uv_connect_cb) -> c_int;    


  pub fn uv_pipe_init(l: *mut uv_loop_t, h: *mut uv_pipe_t, ipc: c_int) -> c_int;
  pub fn uv_pipe_open(h: *mut uv_pipe_t, file: uv_file) -> c_int;
  pub fn uv_pipe_bind(h: *mut uv_pipe_t, name: *const c_char) -> c_int;
  pub fn uv_pipe_connect(r: *mut uv_connect_t, h: *mut uv_pipe_t, name: *const c_char, cb: uv_connect_cb);
  pub fn uv_pipe_getsockname(h: *const uv_pipe_t, b: *mut c_char, s: *mut size_t) -> c_int;
  pub fn uv_pipe_getpeername(h: *const uv_pipe_t, b: *mut c_char, s: *mut size_t) -> c_int;
  pub fn uv_pipe_pending_instances(h: *mut uv_pipe_t, count: c_int);
  pub fn uv_pipe_pending_count(h: *mut uv_pipe_t) -> c_int;
  pub fn uv_pipe_pending_type(h: *mut uv_pipe_t) -> uv_handle_type;
  pub fn uv_pipe_chmod(h: *mut uv_pipe_t, flags: c_int) -> c_int;
  

  pub fn uv_version() -> c_uint;

  pub fn uv_version_string() -> *const c_char;

  pub fn uv_err_name(err: c_int) -> *mut c_char;

  pub fn uv_default_loop() -> *mut uv_loop_t;

  pub fn uv_loop_new() -> *mut uv_loop_t;

  pub fn uv_loop_init(l: *mut uv_loop_t) -> c_int;

  pub fn uv_loop_delete(l: *mut uv_loop_t) -> c_int;

  pub fn uv_loop_close(l: *mut uv_loop_t) -> c_int;

  pub fn uv_loop_alive(l: *const uv_loop_t) -> c_int;

  pub fn uv_now(l: *const uv_loop_t) -> u64;

  pub fn uv_loop_get_data(l: *const uv_loop_t) -> *mut c_void;

  pub fn uv_loop_set_data(l: *const uv_loop_t, d: *mut c_void);

  pub fn uv_run(l: *mut uv_loop_t, m: uv_run_mode) -> c_int;

  pub fn uv_stop(l: *mut uv_loop_t) -> c_int;

  pub fn uv_backend_fd(l: *const uv_loop_t) -> c_int;

  pub fn uv_backend_timeout(l: *const uv_loop_t) -> c_int;


  pub fn uv_setup_args(argc: c_int, argv: *mut *mut c_char) -> *mut *mut c_char;
  pub fn uv_get_process_title(buffer: *mut c_char, size: usize) -> c_int;
  pub fn uv_set_process_title(title: *const c_char) -> c_int;
  pub fn uv_resident_set_memory(rss: *mut usize) -> c_int;
  pub fn uv_uptime(uptime: *mut f64) -> c_int;
  pub fn uv_get_osfhandle(fd: c_int) -> uv_os_fd_t;
  pub fn uv_open_osfhandle(os_fd: uv_os_fd_t) -> c_int;

  pub fn uv_getrusage(rusage: *mut uv_rusage_t) -> c_int;
  pub fn uv_os_homedir(buffer: *mut c_char, size: *mut usize) -> c_int;
  pub fn uv_os_tmpdir(buffer: *mut c_char, size: *mut usize) -> c_int;
  pub fn uv_os_get_passwd(pwd: *mut uv_passwd_t) -> c_int;
  pub fn uv_os_free_passwd(pwd: *mut uv_passwd_t);

  pub fn uv_os_getpid() -> uv_pid_t;
  pub fn uv_os_getppid() -> uv_pid_t;

  pub fn uv_os_getpriority(pid: uv_pid_t, priority: *mut c_int) -> c_int;
  pub fn uv_os_setpriority(pid: uv_pid_t, priority: c_int) -> c_int;

  pub fn uv_cpu_info(cpu_infos: *mut *mut uv_cpu_info_t, count: *mut c_int) -> c_int;
  pub fn uv_free_cpu_info(cpu_infos: *mut uv_cpu_info_t, count: c_int);

  pub fn uv_interface_addresses(addresses: *mut *mut uv_interface_address_t, count: *mut c_int) -> c_int;
  pub fn uv_free_interface_addresses(addresses: *mut uv_interface_address_t, count: c_int);


  pub fn uv_req_size(type_: uv_req_type) -> usize;
  pub fn uv_req_get_data(req: *const uv_req_t) -> *mut c_void;
  pub fn uv_req_set_data(req: *mut uv_req_t, data: *mut c_void);
  pub fn uv_req_get_type(req: *const uv_req_t) -> uv_req_type;
  pub fn uv_req_type_name(req_type: uv_req_type) -> *const c_char;

  
  pub fn uv_print_all_handles(l: *mut uv_loop_t, stream: *mut FILE);
  pub fn uv_print_active_handles(l: *mut uv_loop_t, stream: *mut FILE);

  pub fn uv_buf_init(base: *mut c_char, len: c_uint) -> uv_buf_t;

  pub fn uv_pipe(fds: *mut uv_file, read_flags: c_int, write_flags: c_int) -> c_int;
  pub fn uv_socketpair(ty: c_int, proto: c_int, sock_vec: *mut uv_os_sock_t, flags0: c_int, flags1: c_int) -> c_int;


  pub fn uv_udp_init(l: *mut uv_loop_t, h: *mut uv_udp_t) -> c_int;
  pub fn uv_udp_init_ex(l: *mut uv_loop_t, h: *mut uv_udp_t, flags: c_uint) -> c_int;
  pub fn uv_udp_open(h: *mut uv_udp_t, sock: uv_os_sock_t) -> c_int;
  pub fn uv_udp_bind(h: *mut uv_udp_t, addr: *const sockaddr, flags: c_uint) -> c_int;
  pub fn uv_udp_connect(h: *mut uv_udp_t, addr: *const sockaddr) -> c_int;
  pub fn uv_udp_getpeername(h: *const uv_tcp_t, name: *mut sockaddr, namelen: *mut c_int) -> c_int;
  pub fn uv_udp_getsockname(h: *const uv_tcp_t, name: *mut sockaddr, namelen: *mut c_int) -> c_int;
  pub fn uv_udp_set_membership(h: *mut uv_udp_t, ma: *const multicast_addr, ia: *const interface_addr, m: uv_membership) -> c_int;
  pub fn uv_udp_set_source_membership(
    h: *mut uv_udp_t,
    ma: *const multicast_addr,
    ia: *const interface_addr,
    sa: *const source_addr,
    m: uv_membership
  ) -> c_int;

  pub fn uv_spawn(l: *mut uv_loop_t, h: *mut uv_process_t, o: *const uv_process_options_t) -> c_int;
  pub fn uv_process_kill(h: *mut uv_process_t, s: c_int) -> c_int;
  pub fn uv_process_get_pid(h: *const uv_process_t) -> c_int;

  pub fn uv_kill(p: c_int, s: c_int) -> c_int;  

  pub fn uv_signal_init(l: *mut uv_loop_t, h: *mut uv_signal_t) -> c_int;
  pub fn uv_signal_start(h: *mut uv_signal_t, cb: uv_signal_cb, signum: c_int) -> c_int;
  pub fn uv_signal_start_oneshot(h: *mut uv_signal_t, cb: uv_signal_cb, signum: c_int) -> c_int;
  pub fn uv_signal_stop(h: *mut uv_signal_t) -> c_int;    


  pub fn uv_ref(h: *mut uv_handle_t) -> c_void;
  pub fn uv_unref(h: *mut uv_handle_t) -> c_void;
  pub fn uv_has_ref(h: *const uv_handle_t) -> c_int;  
}

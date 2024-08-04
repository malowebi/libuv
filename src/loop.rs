use std::fmt;
use std::os::raw::c_void;
use crate::error::Error;
use crate::bindings::{
  self, uv_loop_t, uv_loop_option, uv_run_mode,
  UV_LOOP_BLOCK_SIGNAL, UV_METRICS_IDLE_TIME,
  UV_RUN_DEFAULT, UV_RUN_ONCE, UV_RUN_NOWAIT,
};

repr!{
  pub type Loop = uv_loop_t;
}

pub type WalkCb = bindings::uv_walk_cb /* Option<unsafe extern "C" fn(*mut uv_handle_t, *mut c_void)> */;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum LoopOption {
  /// Block a signal when polling for new events.
  /// The second argument to `uv_loop_configure` is the signal number.
  /// This operation is currently only implemented for SIGPROF signals, to suppress unnecessary
  /// wakeups when using a sampling profiler. Reqing other signals will fail with UV_EINVAL.
  LoopBlockSignal = UV_LOOP_BLOCK_SIGNAL,
  /// Accumulate the amount of idle time the event loop spends in the event provider.
  /// This option is necessary to use `uv_metrics_idle_time`.
  MetricsIdleTime = UV_METRICS_IDLE_TIME,
}

impl From<LoopOption> for uv_loop_option {
  fn from(option: LoopOption) -> uv_loop_option {
    match option {
      LoopOption::LoopBlockSignal => UV_LOOP_BLOCK_SIGNAL,
      LoopOption::MetricsIdleTime => UV_METRICS_IDLE_TIME,
    }
  }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum RunMode {
  /// Runs the event loop until there are no more active and referenced handles or requests.
  /// Returns non-zero if uv_stop() was called and there are still active handles or requests.
  /// Returns zero in all other cases.
  Default = UV_RUN_DEFAULT,
  /// Poll for i/o once. Note that this function blocks if there are no pending callbacks.
  /// Returns zero when done (no active handles or requests left), or non-zero if more callbacks
  /// are expected (meaning you should run the event loop again sometime in the future).
  Once = UV_RUN_ONCE,
  /// Poll for i/o once but don’t block if there are no pending callbacks.
  /// Returns zero if done (no active handles or requests left), or non-zero if more callbacks
  /// are expected (meaning you should run the event loop again sometime in the future).
  NoWait = UV_RUN_NOWAIT,
}

impl From<RunMode> for uv_run_mode {
  fn from(mode: RunMode) -> Self {
    match mode {
      RunMode::Default  => UV_RUN_DEFAULT,
      RunMode::Once     => UV_RUN_ONCE,
      RunMode::NoWait   => UV_RUN_NOWAIT,
    }
  }
}

impl Loop {
  pub fn get_default() -> &'static mut Self {
    unsafe {
      let ptr = crate::bindings::uv_default_loop() as *mut Loop;
      ptr.as_mut().unwrap()
    }
  }

  pub fn init(&mut self) -> Result<(), Error> {
    use crate::bindings::*;

    let rc = unsafe {
      uv_loop_init( self.as_mut_ptr() )
    };

    if rc < 0 {
      return Err( Error::from(rc) );
    }

    Ok(())
  }

  pub fn run(&mut self, mode: RunMode) -> Result<(), Error> {
    use crate::bindings::*;

    let rc = unsafe {
      uv_run(self.as_mut_ptr(), mode.into())
    };

    if rc < 0 {
      return Err( Error::from(rc) );
    }

    Ok(())
  }

  pub fn stop(&mut self) {
    use crate::bindings::*;
    unsafe {
      uv_stop( self.as_mut_ptr() )
    }
  }

  pub fn close(&mut self) -> Result<(), Error> {
    use crate::bindings::*;

    let rc = unsafe {
      uv_loop_close( self.as_mut_ptr() )
    };

    if rc < 0 {
      return Err( Error::from(rc) );
    }

    Ok(())
  }

  pub fn alive(&self) -> bool {
    use crate::bindings::*;
    unsafe {
      uv_loop_alive(self.as_ptr()) != 0
    }
  }

  pub fn now(&self) -> u64 {
    use crate::bindings::*;
    unsafe {
      uv_now(self.as_ptr())
    }
  }

  pub fn get_data(&self) -> *mut c_void {
    use crate::bindings::*;
    unsafe {
      uv_loop_get_data(self.as_ptr())
    }
  }

  pub fn set_data<T>(&mut self, data: *mut T) {
    use crate::bindings::*;
    unsafe {
      uv_loop_set_data(self.as_mut_ptr(), data as *mut c_void)
    }
  }  

  /// Walk the list of handles
  /// `walk_cb` will be executed with the given `ctx`.
  #[allow(clippy::not_unsafe_ptr_arg_deref)]
  pub fn walk(&mut self, walk_cb: WalkCb, ctx: *mut c_void) {
    use crate::bindings::*;
    unsafe {
      uv_walk(self.as_mut_ptr(), walk_cb, ctx);
    }
  }
}

impl fmt::Debug for Loop {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Loop")
      .field("active_handles", &self.active_handles)
      .field("handle_queue", &self.handle_queue)
      .field("active_reqs", unsafe { &self.active_reqs.count })
      .field("internal_fields", &self.internal_fields)
      .field("stop_flag", &self.stop_flag)
      .field("flags", &self.flags)
      .field("backend_fd", &self.backend_fd)
      .field("pending_queue", &self.pending_queue)
      .field("watcher_queue", &self.watcher_queue)
      .field("nwatchers", &self.nwatchers)
      .field("nfds", &self.nfds)
      .field("wq", &self.wq)
      .field("closing_handles", &self.closing_handles)
      .field("process_handles", &self.process_handles)
      .field("prepare_handles", &self.prepare_handles)
      .field("check_handles", &self.check_handles)
      .field("idle_handles", &self.idle_handles)
      .field("async_handles", &self.async_handles)
      .field("async_unused", &self.async_unused)
      .field("async_wfd", &self.async_wfd)
      .field("timer_heap", &self.timer_heap)
      .field("timer_counter", &self.timer_counter)
      .field("time", &self.time)
      .field("signal_pipefd", &self.signal_pipefd)
      .field("signal_io_watcher", &self.signal_io_watcher)
      .finish()
  }
}

impl fmt::Display for Loop {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[test]
fn test_layout_loop() {
  use std::mem;
  assert_eq!(
    mem::size_of::<Loop>(),
    mem::size_of::<uv_loop_t>(),
    concat!("Size of: ", stringify!(Loop))
  );
  assert_eq!(
    mem::align_of::<Loop>(),
    mem::align_of::<uv_loop_t>(),
    concat!("Alignment of ", stringify!(Loop))
  );
}    

/*

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_loop_s {
    pub data: *mut ::std::os::raw::c_void,
    pub active_handles: ::std::os::raw::c_uint,
    pub handle_queue: [*mut ::std::os::raw::c_void; 2usize],
    pub active_reqs: uv_loop_s__bindgen_ty_1,
    pub internal_fields: *mut ::std::os::raw::c_void,
    pub stop_flag: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_ulong,
    pub backend_fd: ::std::os::raw::c_int,
    pub pending_queue: [*mut ::std::os::raw::c_void; 2usize],
    pub watcher_queue: [*mut ::std::os::raw::c_void; 2usize],
    pub watchers: *mut *mut uv__io_t,
    pub nwatchers: ::std::os::raw::c_uint,
    pub nfds: ::std::os::raw::c_uint,
    pub wq: [*mut ::std::os::raw::c_void; 2usize],
    pub wq_mutex: uv_mutex_t,
    pub wq_async: uv_async_t,
    pub cloexec_lock: uv_rwlock_t,
    pub closing_handles: *mut uv_handle_t,
    pub process_handles: [*mut ::std::os::raw::c_void; 2usize],
    pub prepare_handles: [*mut ::std::os::raw::c_void; 2usize],
    pub check_handles: [*mut ::std::os::raw::c_void; 2usize],
    pub idle_handles: [*mut ::std::os::raw::c_void; 2usize],
    pub async_handles: [*mut ::std::os::raw::c_void; 2usize],
    pub async_unused: ::std::option::Option<unsafe extern "C" fn()>,
    pub async_io_watcher: uv__io_t,
    pub async_wfd: ::std::os::raw::c_int,
    pub timer_heap: uv_loop_s__bindgen_ty_2,
    pub timer_counter: u64,
    pub time: u64,
    pub signal_pipefd: [::std::os::raw::c_int; 2usize],
    pub signal_io_watcher: uv__io_t,
    pub child_watcher: uv_signal_t,
    pub emfile_fd: ::std::os::raw::c_int,
    pub cf_thread: uv_thread_t,
    pub _cf_reserved: *mut ::std::os::raw::c_void,
    pub cf_state: *mut ::std::os::raw::c_void,
    pub cf_mutex: uv_mutex_t,
    pub cf_sem: uv_sem_t,
    pub cf_signals: [*mut ::std::os::raw::c_void; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union uv_loop_s__bindgen_ty_1 {
    pub unused: *mut ::std::os::raw::c_void,
    pub count: ::std::os::raw::c_uint,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_uv_loop_s__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<uv_loop_s__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(uv_loop_s__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<uv_loop_s__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(uv_loop_s__bindgen_ty_1))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_loop_s__bindgen_ty_2 {
    pub min: *mut ::std::os::raw::c_void,
    pub nelts: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_uv_loop_s__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<uv_loop_s__bindgen_ty_2>(),
        16usize,
        concat!("Size of: ", stringify!(uv_loop_s__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<uv_loop_s__bindgen_ty_2>(),
        8usize,
        concat!("Alignment of ", stringify!(uv_loop_s__bindgen_ty_2))
    );
}
#[test]
fn bindgen_test_layout_uv_loop_s() {
    assert_eq!(
        ::std::mem::size_of::<uv_loop_s>(),
        1072usize,
        concat!("Size of: ", stringify!(uv_loop_s))
    );
    assert_eq!(
        ::std::mem::align_of::<uv_loop_s>(),
        8usize,
        concat!("Alignment of ", stringify!(uv_loop_s))
    );
}
extern "C" {
    #[link_name = "\u{1}__Z16uv_loop_get_data"]
    pub fn uv_loop_get_data(arg1: *const uv_loop_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[link_name = "\u{1}__Z16uv_loop_set_data"]
    pub fn uv_loop_set_data(arg1: *mut uv_loop_t, data: *mut ::std::os::raw::c_void);
}
*/

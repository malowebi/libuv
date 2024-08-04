use std::os::raw::c_void;

use crate::bindings::{
  uv_req_t, uv_req_type, 
  uv_req_get_type, uv_req_get_data, uv_req_set_data,
  UV_UNKNOWN_REQ, UV_REQ, UV_CONNECT, UV_WRITE, 
  UV_SHUTDOWN, UV_UDP_SEND, UV_FS, UV_WORK,
  UV_GETADDRINFO, UV_GETNAMEINFO, UV_RANDOM, UV_REQ_TYPE_MAX
};

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReqType {
  UnknownReq    = /* 00 */ UV_UNKNOWN_REQ,
  Req           = /* 01 */ UV_REQ,
  Connect       = /* 02 */ UV_CONNECT,
  Write         = /* 03 */ UV_WRITE,
  Shutdown      = /* 04 */ UV_SHUTDOWN,
  UdpSend       = /* 05 */ UV_UDP_SEND,
  Fs            = /* 06 */ UV_FS,
  Work          = /* 07 */ UV_WORK,
  GetAddrInfo   = /* 08 */ UV_GETADDRINFO,
  GetNameInfo   = /* 09 */ UV_GETNAMEINFO,
  Random        = /* 10 */ UV_RANDOM,
  ReqTypeMax    = /* 11 */ UV_REQ_TYPE_MAX,
}

impl From<uv_req_type> for ReqType {
  fn from(i: uv_req_type) -> Self {
    match i {
    UV_UNKNOWN_REQ  => /* 00 */ ReqType::UnknownReq,
    UV_REQ          => /* 01 */ ReqType::Req,
    UV_CONNECT      => /* 02 */ ReqType::Connect,
    UV_WRITE        => /* 03 */ ReqType::Write,
    UV_SHUTDOWN     => /* 04 */ ReqType::Shutdown,
    UV_UDP_SEND     => /* 05 */ ReqType::UdpSend,
    UV_FS           => /* 06 */ ReqType::Fs,
    UV_WORK         => /* 07 */ ReqType::Work,
    UV_GETADDRINFO  => /* 08 */ ReqType::GetAddrInfo,
    UV_GETNAMEINFO  => /* 09 */ ReqType::GetNameInfo,
    UV_RANDOM       => /* 10 */ ReqType::Random,
    UV_REQ_TYPE_MAX => /* 11 */ ReqType::ReqTypeMax,
      _             =>  unreachable!()
    }
  }
}

repr!{
  pub type Req = uv_req_t;
  /*
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct uv_req_s {
    pub data: *mut ::std::os::raw::c_void,
    pub loop_: *mut uv_loop_t,
    pub type_: uv_req_type,
    pub close_cb: uv_close_cb,
    pub req_queue: [*mut ::std::os::raw::c_void; 2usize],
    pub u: uv_req_s__bindgen_ty_1,
    pub next_closing: *mut uv_req_t,
    pub flags: ::std::os::raw::c_uint,
  }

  #[repr(C)]
  #[derive(Copy, Clone)]
  pub union uv_req_s__bindgen_ty_1 {
      pub fd: ::std::os::raw::c_int,
      pub reserved: [*mut ::std::os::raw::c_void; 4usize],
      _bindgen_union_align: [u64; 4usize],
  }
  extern "C" {
      #[link_name = "\u{1}__Z14uv_req_size"]
      pub fn uv_req_size(type_: uv_req_type) -> size_t;
  }
  extern "C" {
      #[link_name = "\u{1}__Z19uv_req_type_name"]
      pub fn uv_req_type_name(type_: uv_req_type) -> *const ::std::os::raw::c_char;
  }
  extern "C" {
      #[link_name = "\u{1}__Z18uv_req_get_loop"]
      pub fn uv_req_get_loop(req: *const uv_req_t) -> *mut uv_loop_t;
  }
  */  
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub trait ReqImpl {
  fn as_request(&self) -> &Req;
  fn as_mut_request(&mut self) -> &mut Req;

  fn get_type(&self) -> ReqType {
    unsafe {
      let ty = uv_req_get_type(self.as_request().as_ptr());
      ReqType::from(ty)
    }
  }

  fn get_data(&self) -> *mut c_void {
    unsafe {
      uv_req_get_data(self.as_request().as_ptr())
    }
  }

  fn set_data(&mut self, data: *mut c_void) {
    unsafe {
      uv_req_set_data(self.as_mut_request().as_mut_ptr(), data)
    }
  }
}

impl ReqImpl for Req {
  fn as_request(&self) -> &Req {
    self
  }
  
  fn as_mut_request(&mut self) -> &mut Req {
    self
  }
}

pub mod write;
pub use self::write::{Write, WriteCb};

pub mod connect;
pub use self::connect::{Connect, ConnectCb};

pub mod shutdown;
pub use self::shutdown::{Shutdown, ShutdownCb};

#[test]
fn test_layout_req_type() {
  use std::mem;
  assert_eq!(
    mem::size_of::<ReqType>(),
    mem::size_of::<uv_req_type>(),
    concat!("Size of: ", stringify!(ReqType))
  );
  assert_eq!(
    mem::align_of::<ReqType>(),
    mem::align_of::<uv_req_type>(),
    concat!("Alignment of ", stringify!(ReqType))
  );
}

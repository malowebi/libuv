#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]

pub use std::ptr;
pub use std::mem;
pub use std::os::raw::{
  c_void, 
  c_int, c_uint, 
  c_char, c_uchar, 
  c_long, c_ulong, 
  c_short, c_ushort, 
  c_longlong, c_ulonglong,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

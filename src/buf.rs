#![allow(clippy::len_without_is_empty)]

use std::fmt;
use std::mem;
use std::convert::From;
use std::os::raw::{c_char, c_uint};

use crate::error::Error;
use crate::bindings::{
  size_t, uv_buf_t, uv_buf_init
};

repr!{
  pub type Buf = uv_buf_t;
}

impl From<&[u8]> for Buf {
  fn from(bytes: &[u8]) -> Self {
    Buf {
      base: bytes.as_ptr() as *mut i8,
      len:  bytes.len()    as size_t
    }     
  }
}

impl Buf {
  pub fn alloc(base: *mut c_char, len: usize) -> Result<Box<Buf>, Error> {
    let buf = Box::new(
      Buf {
        base, 
        len: len as size_t
      }
    );
    Ok(buf)
  }

  #[inline(always)]
  pub fn realloc(&mut self, base: *mut c_char, len: usize) {
    self.base = base;
    self.len  = len as size_t;
  }

  #[inline(always)]
  pub fn ptr(&self) -> *const Self {
    self as *const Self
  }

  #[inline(always)]
  pub fn ptr_mut(&mut self) -> *mut Self {
    self as *mut Self
  }

  #[inline(always)]
  pub fn base(&self) -> *mut c_char {
    self.base
  }

  #[inline(always)]
  pub fn set_base(&mut self, base: *mut c_char) {
    self.base = base;
  }

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  #[inline(always)]
    pub fn len(&self) -> usize {
    self.len as usize
  }

  #[inline(always)]
  pub fn set_len(&mut self, len: usize) {
    self.len = len as size_t;
  }
}

// impl Drop for Buf {
//   fn drop(&mut self) {
//     // if self.0.base.is_null() || (self.0.len == 0) {
//     //   return;
//     // }

//     // unsafe {
//     //   libc::memset(self.base() as *mut c_void, 0, self.len());
//     //   libc::free(self.0.base as *mut c_void);
//     // };
//   }
// }

// impl fmt::Debug for Buf {
//   fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
//     fmt
//       .debug_struct("Buf")
//       .field("ptr", &self.ptr())
//       .field("len", &self.len())
//       .finish()
//   }
// }

impl fmt::Display for Buf {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    write!(fmt, "Buf({:?}, {})", self.ptr(), self.len())
  }
}

pub fn new(base: *mut c_char, len: usize) -> Result<*mut uv_buf_t, Error> {
  #![allow(clippy::not_unsafe_ptr_arg_deref)]
  unsafe {
    let ptr = libc::calloc(1, mem::size_of::<uv_buf_t>()) as *mut uv_buf_t;
    if ptr.is_null() {
      unimplemented!();
    }
    *ptr = uv_buf_init(base, len as c_uint);
    Ok(ptr)
  }
}

#[inline(always)]
pub fn base(buf: *mut uv_buf_t) -> *mut c_char {
  #![allow(clippy::not_unsafe_ptr_arg_deref)]
  unsafe { (*buf).base }
}

#[inline(always)]
pub fn len(buf: *mut uv_buf_t) -> usize {
  #![allow(clippy::not_unsafe_ptr_arg_deref)]
  unsafe { (*buf).len as usize }
}

#[test]
fn test_layout_buf() {
  use std::mem;
  assert_eq!(
    mem::size_of::<Buf>(),
    mem::size_of::<uv_buf_t>(),
    concat!("Size of: ", stringify!(Buf))
  );
  assert_eq!(
    mem::align_of::<Buf>(),
    mem::align_of::<uv_buf_t>(),
    concat!("Alignment of ", stringify!(Buf))
  );
}

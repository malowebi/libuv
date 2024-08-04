use bitflags::bitflags;

use crate::bindings::{
  uv_stdio_container_t,
  uv_stdio_flags,
  uv_file,
  UV_IGNORE,
  UV_CREATE_PIPE,
  UV_INHERIT_FD,
  UV_INHERIT_STREAM,
  UV_READABLE_PIPE,
  UV_WRITABLE_PIPE,
  UV_NONBLOCK_PIPE,
  UV_OVERLAPPED_PIPE,
};

use crate::stream::Stream;

pub type Fd = uv_file;

bitflags! {
  pub struct StdioFlags: uv_stdio_flags {
    const IGNORE          = UV_IGNORE;
    const CREATE_PIPE     = UV_CREATE_PIPE;
    const INHERIT_FD      = UV_INHERIT_FD;
    const INHERIT_STREAM  = UV_INHERIT_STREAM;
    const READABLE_PIPE   = UV_READABLE_PIPE;
    const WRITABLE_PIPE   = UV_WRITABLE_PIPE;
    const NONBLOCK_PIPE   = UV_NONBLOCK_PIPE;
    const OVERLAPPED_PIPE = UV_OVERLAPPED_PIPE;
  }
}

repr!{
  pub type StdioContainer = uv_stdio_container_t;
}

#[derive(Default)]    
pub struct StdioContainerBuilder {
  container: StdioContainer
}

// impl Default for StdioContainerBuilder {
//   #[inline]
//   fn default() -> Self {
//     Self {
//       container: StdioContainer::default()
//     }
//   }
// }

impl StdioContainerBuilder {
  pub fn flags(mut self, flags: StdioFlags) -> Self {
    self.container.flags = flags.into();
    self
  }

  pub fn fd(mut self, fd: Fd) -> Self {
    self.container.data.fd = fd;
    self
  }

  pub fn stream(mut self, stream: &mut Stream) -> Self {
    self.container.data.stream = stream.as_mut_ptr();
    self
  }

  pub fn build(self) -> StdioContainer {
    self.container
  } 
}

impl StdioContainer {
  pub fn builder() -> StdioContainerBuilder {
    StdioContainerBuilder::default()
  }
}


impl From<StdioFlags> for uv_stdio_flags {
  fn from(evt: StdioFlags) -> uv_stdio_flags {
    evt.bits as uv_stdio_flags
  }
}

impl From<StdioFlags> for i32 {
  fn from(evt: StdioFlags) -> Self {
    evt.bits as i32
  }
}

impl From<uv_stdio_flags> for StdioFlags {
  fn from(i: uv_stdio_flags) -> Self {
    StdioFlags::from_bits(i).unwrap()
  }
}

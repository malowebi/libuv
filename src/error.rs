pub use crate::bindings::*;
pub use crate::thiserror::Error as ThisError;

#[repr(i32)]
#[derive(ThisError, Debug, Clone, Copy, PartialEq)]
pub enum Error {
  #[error("Operation not permitted (UV_EPERM)")]
  EPERM,
  #[error("No such file or directory (UV_ENOENT)")]
  ENOENT,
  #[error("No such process (UV_ESRCH)")]
  ESRCH,
  #[error("Interrupted system call (UV_EINTR)")]
  EINTR,
  #[error("Input/output error (UV_EIO)")]
  EIO,
  #[error("Device not configured (UV_ENXIO)")]
  ENXIO,
  #[error("Argument list too long (UV_E2BIG)")]
  E2BIG,
  #[error("Bad file descriptor (UV_EBADF)")]
  EBADF,
  #[error("Cannot allocate memory (UV_ENOMEM)")]
  ENOMEM,
  #[error("Permission denied (UV_EACCES)")]
  EACCES,
  #[error("Bad address (UV_EFAULT)")]
  EFAULT,
  #[error("Block device required (UV_ENOTBLK)")]
  ENOTBLK,
  #[error("Device / Resource busy (UV_EBUSY)")]
  EBUSY,
  #[error("File exists (UV_EEXIST)")]
  EEXIST,
  #[error("Cross-device link (UV_EXDEV)")]
  EXDEV,
  #[error("Operation not supported by device (UV_ENODEV)")]
  ENODEV,
  #[error("Not a directory (UV_ENOTDIR)")]
  ENOTDIR,
  #[error("Is a directory (UV_EISDIR)")]
  EISDIR,
  #[error("Invalid argument (UV_EINVAL)")]
  EINVAL,
  #[error("Too many open files in system (UV_ENFILE)")]
  ENFILE,
  #[error("Too many open files (UV_EMFILE)")]
  EMFILE,
  #[error("Inappropriate ioctl for device (UV_ENOTTY)")]
  ENOTTY,
  #[error("Text file busy (UV_ETXTBSY)")]
  ETXTBSY,
  #[error("File too large (UV_EFBIG)")]
  EFBIG,
  #[error("No space left on device (UV_ENOSPC)")]
  ENOSPC,
  #[error("Illegal seek (UV_ESPIPE)")]
  ESPIPE,
  #[error("Read-only file system (UV_EROFS)")]
  EROFS,
  #[error("Too many links (UV_EMLINK)")]
  EMLINK,
  #[error("Broken pipe (UV_EPIPE)")]
  EPIPE,
  #[error("Numerical argument out of domain (UV_EDOM)")]
  EDOM,
  #[error("Result too large (UV_ERANGE)")]
  ERANGE,
  #[error("Resource temporarily unavailable (UV_EAGAIN)")]
  EAGAIN,
  #[error("Operation now in progress (UV_EINPROGRESS)")]
  EINPROGRESS,
  #[error("Operation already in progress (UV_EALREADY)")]
  EALREADY,
  #[error("Socket operation on non-socket (UV_ENOTSOCK)")]
  ENOTSOCK,
  #[error("Destination address required (UV_EDESTADDRREQ)")]
  EDESTADDRREQ,
  #[error("Message too long (UV_EMSGSIZE)")]
  EMSGSIZE,
  #[error("Protocol wrong type for socket (UV_EPROTOTYPE)")]
  EPROTOTYPE,
  #[error("Protocol not available (UV_ENOPROTOOPT)")]
  ENOPROTOOPT,
  #[error("Protocol not supported (UV_EPROTONOSUPPORT)")]
  EPROTONOSUPPORT,
  #[error("Socket type not supported (UV_ESOCKTNOSUPPORT)")]
  ESOCKTNOSUPPORT,
  #[error("Operation not supported (UV_ENOTSUP)")]
  ENOTSUP,
  #[error("Protocol family not supported (UV_EPFNOSUPPORT)")]
  EPFNOSUPPORT,
  #[error("Address family not supported by protocol family (UV_EAFNOSUPPORT)")]
  EAFNOSUPPORT,
  #[error("Address already in use (UV_EADDRINUSE)")]
  EADDRINUSE,
  #[error("Can't assign requested address (UV_EADDRNOTAVAIL)")]
  EADDRNOTAVAIL,
  #[error("Network is down (UV_ENETDOWN)")]
  ENETDOWN,
  #[error("Network is unreachable (UV_ENETUNREACH)")]
  ENETUNREACH,
  #[error("Network dropped connection on reset (UV_ENETRESET)")]
  ENETRESET,
  #[error("Software caused connection abort (UV_ECONNABORTED)")]
  ECONNABORTED,
  #[error("Connection reset by peer (UV_ECONNRESET)")]
  ECONNRESET,
  #[error("No buffer space available (UV_ENOBUFS)")]
  ENOBUFS,
  #[error("Socket is already connected (UV_EISCONN)")]
  EISCONN,
  #[error("Socket is not connected (UV_ENOTCONN)")]
  ENOTCONN,
  #[error("Can't send after socket shutdown (UV_ESHUTDOWN)")]
  ESHUTDOWN,
  #[error("Too many references: can't splice (UV_ETOOMANYREFS)")]
  ETOOMANYREFS,
  #[error("Operation timed out (UV_ETIMEDOUT)")]
  ETIMEDOUT,
  #[error("Connection refused (UV_ECONNREFUSED)")]
  ECONNREFUSED,
  #[error("Too many levels of symbolic links (UV_ELOOP)")]
  ELOOP,
  #[error("File name too long (UV_ENAMETOOLONG)")]
  ENAMETOOLONG,
  #[error("Host is down (UV_EHOSTDOWN)")]
  EHOSTDOWN,
  #[error("No route to host (UV_EHOSTUNREACH)")]
  EHOSTUNREACH,
  #[error("Directory not empty (UV_ENOTEMPTY)")]
  ENOTEMPTY,
  #[error("Too many processes (UV_EPROCLIM)")]
  EPROCLIM,
  #[error("Too many users (UV_EUSERS)")]
  EUSERS,
  #[error("Disc quota exceeded (UV_EDQUOT)")]
  EDQUOT,
  #[error("Stale NFS file handle (UV_ESTALE)")]
  ESTALE,
  #[error("Too many levels of remote in path (UV_EREMOTE)")]
  EREMOTE,
  #[error("RPC struct is bad (UV_EBADRPC)")]
  EBADRPC,
  #[error("RPC version wrong (UV_ERPCMISMATCH)")]
  ERPCMISMATCH,
  #[error("RPC prog. not avail (UV_EPROGUNAVAIL)")]
  EPROGUNAVAIL,
  #[error("Program version wrong (UV_EPROGMISMATCH)")]
  EPROGMISMATCH,
  #[error("Bad procedure for program (UV_EPROCUNAVAIL)")]
  EPROCUNAVAIL,
  #[error("No locks available (UV_ENOLCK)")]
  ENOLCK,
  #[error("Function not implemented (UV_ENOSYS)")]
  ENOSYS,
  #[error("Inappropriate file type or format (UV_EFTYPE)")]
  EFTYPE,
  #[error("Authentication error (UV_EAUTH)")]
  EAUTH,
  #[error("Need authenticator (UV_ENEEDAUTH)")]
  ENEEDAUTH,
  #[error("Device power is off (UV_EPWROFF)")]
  EPWROFF,
  #[error("Device error, e.g. paper out (UV_EDEVERR)")]
  EDEVERR,
  #[error("Value too large to be stored in data type (UV_EOVERFLOW)")]
  EOVERFLOW,
  #[error("Bad executable (UV_EBADEXEC)")]
  EBADEXEC,
  #[error("Bad CPU type in executable (UV_EBADARCH)")]
  EBADARCH,
  #[error("Shared library version mismatch (UV_ESHLIBVERS)")]
  ESHLIBVERS,
  #[error("Malformed Macho file (UV_EBADMACHO)")]
  EBADMACHO,
  #[error("Operation canceled (UV_ECANCELED)")]
  ECANCELED,
  #[error("Identifier removed (UV_EIDRM)")]
  EIDRM,
  #[error("No message of desired type (UV_ENOMSG)")]
  ENOMSG,
  #[error("Illegal byte sequence (UV_EILSEQ)")]
  EILSEQ,
  #[error("Attribute not found (UV_ENOATTR)")]
  ENOATTR,
  #[error("Bad message (UV_EBADMSG)")]
  EBADMSG,
  #[error("Reserved (UV_EMULTIHOP)")]
  EMULTIHOP,
  #[error("No message available on STREAM (UV_ENODATA)")]
  ENODATA,
  #[error("Reserved (UV_ENOLINK)")]
  ENOLINK,
  #[error("No STREAM resources (UV_ENOSR)")]
  ENOSR,
  #[error("Not a STREAM (UV_ENOSTR)")]
  ENOSTR,
  #[error("Protocol error (UV_EPROTO)")]
  EPROTO,
  #[error("STREAM ioctl timeout (UV_ETIME)")]
  ETIME,
  #[error("Operation not supported on socket (UV_EOPNOTSUPP)")]
  EOPNOTSUPP,
  #[error("No such policy registered (UV_ENOPOLICY)")]
  ENOPOLICY,
  #[error("State not recoverable (UV_ENOTRECOVERABLE)")]
  ENOTRECOVERABLE,
  #[error("Previous owner died (UV_EOWNERDEAD)")]
  EOWNERDEAD,
  #[error("Interface output queue is full (UV_EQFULL)")]
  EQFULL,
  #[error("Must be equal largest errno (UV_MAX)")]
  MAX,
}

impl From<Error> for i32 {
  fn from(err: Error) -> Self {
    use Error::*;
    match err {
      EPERM => UV_EPERM,
      ENOENT => UV_ENOENT,
      ESRCH => UV_ESRCH,
      EINTR => UV_EINTR,
      EIO => UV_EIO,
      ENXIO => UV_ENXIO,
      E2BIG => UV_E2BIG,
      EBADF => UV_EBADF,
      ENOMEM => UV_ENOMEM,
      EACCES => UV_EACCES,
      EFAULT => UV_EFAULT,
      EBUSY => UV_EBUSY,
      EEXIST => UV_EEXIST,
      EXDEV => UV_EXDEV,
      ENODEV => UV_ENODEV,
      ENOTDIR => UV_ENOTDIR,
      EISDIR => UV_EISDIR,
      EINVAL => UV_EINVAL,
      ENFILE => UV_ENFILE,
      EMFILE => UV_EMFILE,
      ENOTTY => UV_ENOTTY,
      ETXTBSY => UV_ETXTBSY,
      EFBIG => UV_EFBIG,
      ENOSPC => UV_ENOSPC,
      ESPIPE => UV_ESPIPE,
      EROFS => UV_EROFS,
      EMLINK => UV_EMLINK,
      EPIPE => UV_EPIPE,
      ERANGE => UV_ERANGE,
      EAGAIN => UV_EAGAIN,
      EALREADY => UV_EALREADY,
      ENOTSOCK => UV_ENOTSOCK,
      EDESTADDRREQ => UV_EDESTADDRREQ,
      EMSGSIZE => UV_EMSGSIZE,
      EPROTOTYPE => UV_EPROTOTYPE,
      ENOPROTOOPT => UV_ENOPROTOOPT,
      EPROTONOSUPPORT => UV_EPROTONOSUPPORT,
      ESOCKTNOSUPPORT => UV_ESOCKTNOSUPPORT,
      ENOTSUP => UV_ENOTSUP,
      EAFNOSUPPORT => UV_EAFNOSUPPORT,
      EADDRINUSE => UV_EADDRINUSE,
      EADDRNOTAVAIL => UV_EADDRNOTAVAIL,
      ENETDOWN => UV_ENETDOWN,
      ENETUNREACH => UV_ENETUNREACH,
      ECONNABORTED => UV_ECONNABORTED,
      ECONNRESET => UV_ECONNRESET,
      ENOBUFS => UV_ENOBUFS,
      EISCONN => UV_EISCONN,
      ENOTCONN => UV_ENOTCONN,
      ESHUTDOWN => UV_ESHUTDOWN,
      ETIMEDOUT => UV_ETIMEDOUT,
      ECONNREFUSED => UV_ECONNREFUSED,
      ELOOP => UV_ELOOP,
      ENAMETOOLONG => UV_ENAMETOOLONG,
      EHOSTDOWN => UV_EHOSTDOWN,
      EHOSTUNREACH => UV_EHOSTUNREACH,
      ENOTEMPTY => UV_ENOTEMPTY,
      ENOSYS => UV_ENOSYS,
      EFTYPE => UV_EFTYPE,
      EOVERFLOW => UV_EOVERFLOW,
      ECANCELED => UV_ECANCELED,
      EILSEQ => UV_EILSEQ,
      EPROTO => UV_EPROTO,
      MAX => UV_ERRNO_MAX,
      _ => unimplemented!()
    } 
  }
}

impl From<i32> for Error {
  #[allow(clippy::neg_multiply)]
  fn from(i: i32) -> Self {
    let err_code = i * -1;

    match i {
      UV_EPERM           => Error::EPERM,
      UV_ENOENT          => Error::ENOENT,
      UV_ESRCH           => Error::ESRCH,
      UV_EINTR           => Error::EINTR,
      UV_EIO             => Error::EIO,
      UV_ENXIO           => Error::ENXIO,
      UV_E2BIG           => Error::E2BIG,
      UV_EBADF           => Error::EBADF,
      UV_ENOMEM          => Error::ENOMEM,
      UV_EACCES          => Error::EACCES,
      UV_EFAULT          => Error::EFAULT,
      UV_EBUSY           => Error::EBUSY,
      UV_EEXIST          => Error::EEXIST,
      UV_EXDEV           => Error::EXDEV,
      UV_ENODEV          => Error::ENODEV,
      UV_ENOTDIR         => Error::ENOTDIR,
      UV_EISDIR          => Error::EISDIR,
      UV_EINVAL          => Error::EINVAL,
      UV_ENFILE          => Error::ENFILE,
      UV_EMFILE          => Error::EMFILE,
      UV_ENOTTY          => Error::ENOTTY,
      UV_ETXTBSY         => Error::ETXTBSY,
      UV_EFBIG           => Error::EFBIG,
      UV_ENOSPC          => Error::ENOSPC,
      UV_ESPIPE          => Error::ESPIPE,
      UV_EROFS           => Error::EROFS,
      UV_EMLINK          => Error::EMLINK,
      UV_EPIPE           => Error::EPIPE,
      UV_ERANGE          => Error::ERANGE,
      UV_EAGAIN          => Error::EAGAIN,
      UV_EALREADY        => Error::EALREADY,
      UV_ENOTSOCK        => Error::ENOTSOCK,
      UV_EDESTADDRREQ    => Error::EDESTADDRREQ,
      UV_EMSGSIZE        => Error::EMSGSIZE,
      UV_EPROTOTYPE      => Error::EPROTOTYPE,
      UV_ENOPROTOOPT     => Error::ENOPROTOOPT,
      UV_EPROTONOSUPPORT => Error::EPROTONOSUPPORT,
      UV_ESOCKTNOSUPPORT => Error::ESOCKTNOSUPPORT,
      UV_ENOTSUP         => Error::ENOTSUP,
      UV_EAFNOSUPPORT    => Error::EAFNOSUPPORT,
      UV_EADDRINUSE      => Error::EADDRINUSE,
      UV_EADDRNOTAVAIL   => Error::EADDRNOTAVAIL,
      UV_ENETDOWN        => Error::ENETDOWN,
      UV_ENETUNREACH     => Error::ENETUNREACH,
      UV_ECONNABORTED    => Error::ECONNABORTED,
      UV_ECONNRESET      => Error::ECONNRESET,
      UV_ENOBUFS         => Error::ENOBUFS,
      UV_EISCONN         => Error::EISCONN,
      UV_ENOTCONN        => Error::ENOTCONN,
      UV_ESHUTDOWN       => Error::ESHUTDOWN,
      UV_ETIMEDOUT       => Error::ETIMEDOUT,
      UV_ECONNREFUSED    => Error::ECONNREFUSED,
      UV_ELOOP           => Error::ELOOP,
      UV_ENAMETOOLONG    => Error::ENAMETOOLONG,
      UV_EHOSTDOWN       => Error::EHOSTDOWN,
      UV_EHOSTUNREACH    => Error::EHOSTUNREACH,
      UV_ENOTEMPTY       => Error::ENOTEMPTY,
      UV_ENOSYS          => Error::ENOSYS,
      UV_EFTYPE          => Error::EFTYPE,
      UV_EOVERFLOW       => Error::EOVERFLOW,
      UV_ECANCELED       => Error::ECANCELED,
      UV_EILSEQ          => Error::EILSEQ,
      UV_EPROTO          => Error::EPROTO,
      _                      => match err_code {
        UV_ERRNO_MAX => Error::MAX,
        _                 => unreachable!()
      }
    }
  }
}

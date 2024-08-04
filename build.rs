extern crate cc;
extern crate bindgen;

use std::env;
use std::io::Write;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

fn main() {
  let vendor_path   = vendor_path();
  let include_path  = vendor_path.join("include");
  build_static_lib(&vendor_path);
  generate_bindings(&include_path);

  println!("cargo:include={}", include_path.to_string_lossy());
  println!("cargo:rustc-link-search=native={}", env::var_os("OUT_DIR").unwrap().to_str().unwrap());
  println!("cargo:rustc-link-lib=static=uv");
  println!("cargo:rerun-if-changed=build.rs");  
  println!("cargo:rerun-if-changed=sys");  
}

fn vendor_path() -> PathBuf {
  let mut cwd = std::env::current_dir().unwrap();
  cwd.push("sys");
  cwd
}

// https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt
fn build_static_lib<P: AsRef<Path>>(vendor_path: &P) {
  let mut build = cc::Build::new();

  dbg!(&build);

  let src_path  = vendor_path.as_ref().join("src");
  let win_path  = src_path.join("win");
  let unix_path = src_path.join("unix");

  let target      = env::var("TARGET").unwrap();
  let is_apple      = target.contains("-apple-");
  let is_dragonfly  = target.ends_with("-dragonfly");
  let is_freebsd    = target.ends_with("-freebsd");
  let is_linux      = target.contains("-linux-");
  let is_netbsd     = target.ends_with("-netbsd");
  let is_openbsd    = target.ends_with("-openbsd");
  let is_solaris    = target.ends_with("-solaris");

  let compiler  = build.get_compiler();
  let is_clang  = compiler.is_like_clang();
  let is_gnu    = compiler.is_like_gnu();
  let is_msvc   = compiler.is_like_msvc();

  build
    .include(vendor_path.as_ref().join("include"))
    .include(&src_path);

  if is_msvc {
    // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L102,L113
    // check_c_compiler_flag(/W4 UV_LINT_W4)
    // check_c_compiler_flag(/wd4100 UV_LINT_NO_UNUSED_PARAMETER_MSVC)
    // check_c_compiler_flag(/wd4127 UV_LINT_NO_CONDITIONAL_CONSTANT_MSVC)
    // check_c_compiler_flag(/wd4201 UV_LINT_NO_NONSTANDARD_MSVC)
    // check_c_compiler_flag(/wd4206 UV_LINT_NO_NONSTANDARD_EMPTY_TU_MSVC)
    // check_c_compiler_flag(/wd4210 UV_LINT_NO_NONSTANDARD_FILE_SCOPE_MSVC)
    // check_c_compiler_flag(/wd4232 UV_LINT_NO_NONSTANDARD_NONSTATIC_DLIMPORT_MSVC)
    // check_c_compiler_flag(/wd4456 UV_LINT_NO_HIDES_LOCAL)
    // check_c_compiler_flag(/wd4457 UV_LINT_NO_HIDES_PARAM)
    // check_c_compiler_flag(/wd4459 UV_LINT_NO_HIDES_GLOBAL)
    // check_c_compiler_flag(/wd4706 UV_LINT_NO_CONDITIONAL_ASSIGNMENT_MSVC)
    // check_c_compiler_flag(/wd4996 UV_LINT_NO_UNSAFE_MSVC)  
    build
      .flag("/W4")      // UV_LINT_W4
      .flag("/wd4100")  // UV_LINT_NO_UNUSED_PARAMETER_MSVC
      .flag("/wd4127")  // UV_LINT_NO_CONDITIONAL_CONSTANT_MSVC
      .flag("/wd4201")  // UV_LINT_NO_NONSTANDARD_MSVC
      .flag("/wd4206")  // UV_LINT_NO_NONSTANDARD_EMPTY_TU_MSVC
      .flag("/wd4210")  // UV_LINT_NO_NONSTANDARD_FILE_SCOPE_MSVC
      .flag("/wd4232")  // UV_LINT_NO_NONSTANDARD_NONSTATIC_DLIMPORT_MSVC
      .flag("/wd4456")  // UV_LINT_NO_HIDES_LOCAL
      .flag("/wd4457")  // UV_LINT_NO_HIDES_PARAM
      .flag("/wd4459")  // UV_LINT_NO_HIDES_GLOBAL
      .flag("/wd4706")  // UV_LINT_NO_CONDITIONAL_ASSIGNMENT_MSVC
      .flag("/wd4996"); // UV_LINT_NO_UNSAFE_MSVC
    
    // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L122
    // [cmake] check_c_compiler_flag(/utf-8 UV_LINT_UTF8_MSVC)
    build.flag("/utf-8"); // UV_LINT_UTF8_MSVC
  }
  else if is_apple || is_clang || is_gnu {
    // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L20
    // [cmake] set(CMAKE_C_VISIBILITY_PRESET hidden)
    build.flag("-fvisibility=hidden");
    // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L23
    // [cmake] set(CMAKE_C_STANDARD 90)
    build.flag("--std=gnu89");

    // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L115
    // [cmake] check_c_compiler_flag(-Wall UV_LINT_WALL)
    build.flag("-Wall");

    // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L118,L120
    // check_c_compiler_flag(-Wno-unused-parameter UV_LINT_NO_UNUSED_PARAMETER)
    // check_c_compiler_flag(-Wstrict-prototypes UV_LINT_STRICT_PROTOTYPES)
    // check_c_compiler_flag(-Wextra UV_LINT_EXTRA)
    build      
      .flag("-Wstrict-prototypes")
      .flag("-Wextra")
      .flag("-Wno-unused-parameter");
  }

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L161,L162
  // check_c_compiler_flag(-fno-strict-aliasing UV_F_STRICT_ALIASING)
  // list(APPEND uv_cflags $<$<BOOL:${UV_F_STRICT_ALIASING}>:-fno-strict-aliasing>)
  if is_gnu {
    build.flag("-fno-strict-aliasing");
  }

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L164,L176
  // set(uv_sources
  //   src/fs-poll.c
  //   src/idna.c
  //   src/inet.c
  //   src/random.c
  //   src/strscpy.c
  //   src/strtok.c
  //   src/thread-common.c
  //   src/threadpool.c
  //   src/timer.c
  //   src/uv-common.c
  //   src/uv-data-getter-setters.c
  //   src/version.c)
  build
    .file(src_path.join("fs-poll.c"))
    .file(src_path.join("idna.c"))
    .file(src_path.join("inet.c"))
    .file(src_path.join("random.c"))
    .file(src_path.join("strscpy.c"))
    .file(src_path.join("strtok.c"))
    .file(src_path.join("threadpool.c"))
    .file(src_path.join("timer.c"))
    .file(src_path.join("uv-common.c"))
    .file(src_path.join("uv-data-getter-setters.c"))
    .file(src_path.join("version.c"));

  if cfg!(windows) {
    // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L179
    // list(APPEND uv_defines WIN32_LEAN_AND_MEAN _WIN32_WINNT=0x0602)
    build
      .define("WIN32_LEAN_AND_MEAN", None)
      .define("_WIN32_WINNT", "0x0602");

    // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L180,L189
    // list(APPEND uv_libraries
    //   psapi
    //   user32
    //   advapi32
    //   iphlpapi
    //   userenv
    //   ws2_32
    //   dbghelp
    //   ole32
    //   uuid)
    println!("cargo:rustc-link-lib=psapi");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=iphlpapi");
    println!("cargo:rustc-link-lib=userenv");
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-lib=dbghelp");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=uuid");

    // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L190,L215
    // list(APPEND uv_sources
    //   src/win/async.c
    //   src/win/core.c
    //   src/win/detect-wakeup.c
    //   src/win/dl.c
    //   src/win/error.c
    //   src/win/fs.c
    //   src/win/fs-event.c
    //   src/win/getaddrinfo.c
    //   src/win/getnameinfo.c
    //   src/win/handle.c
    //   src/win/loop-watcher.c
    //   src/win/pipe.c
    //   src/win/thread.c
    //   src/win/poll.c
    //   src/win/process.c
    //   src/win/process-stdio.c
    //   src/win/signal.c
    //   src/win/snprintf.c
    //   src/win/stream.c
    //   src/win/tcp.c
    //   src/win/tty.c
    //   src/win/udp.c
    //   src/win/util.c
    //   src/win/winapi.c
    //   src/win/winsock.c)
    build
      .file(win_path.join("async.c"))
      .file(win_path.join("core.c"))
      .file(win_path.join("detect-wakeup.c"))
      .file(win_path.join("dl.c"))
      .file(win_path.join("error.c"))
      .file(win_path.join("fs.c"))
      .file(win_path.join("fs-event.c"))
      .file(win_path.join("getaddrinfo.c"))
      .file(win_path.join("getnameinfo.c"))
      .file(win_path.join("handle.c"))
      .file(win_path.join("loop-watcher.c"))
      .file(win_path.join("pipe.c"))
      .file(win_path.join("thread.c"))
      .file(win_path.join("poll.c"))
      .file(win_path.join("process.c"))
      .file(win_path.join("process-stdio.c"))
      .file(win_path.join("signal.c"))
      .file(win_path.join("snprintf.c"))
      .file(win_path.join("stream.c"))
      .file(win_path.join("tcp.c"))
      .file(win_path.join("tty.c"))
      .file(win_path.join("udp.c"))
      .file(win_path.join("util.c"))
      .file(win_path.join("winapi.c"))
      .file(win_path.join("winsock.c"));
  }
  else {
    // CMakeLists.txt checks that the target platform is not Android nor OS/390 
    // nor QNX before trying to link to the pthread library. We support only
    // "standard" unix platforms for the moment so we can safely assume that 
    // phtread is always available for the configured target environment.
    println!("cargo:rustc-link-lib=pthread");

    // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L219
    // list(APPEND uv_defines _FILE_OFFSET_BITS=64 _LARGEFILE_SOURCE)
    build
    .define("_FILE_OFFSET_BITS", "64")
    .define("_LARGEFILE_SOURCE", None);

    // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L226,L244
    // list(APPEND uv_sources
    //   src/unix/async.c
    //   src/unix/core.c
    //   src/unix/dl.c
    //   src/unix/fs.c
    //   src/unix/getaddrinfo.c
    //   src/unix/getnameinfo.c
    //   src/unix/loop-watcher.c
    //   src/unix/loop.c
    //   src/unix/pipe.c
    //   src/unix/poll.c
    //   src/unix/process.c
    //   src/unix/random-devurandom.c
    //   src/unix/signal.c
    //   src/unix/stream.c
    //   src/unix/tcp.c
    //   src/unix/thread.c
    //   src/unix/tty.c
    //   src/unix/udp.c)
    build
      .file(unix_path.join("async.c"))
      .file(unix_path.join("core.c"))
      .file(unix_path.join("dl.c"))
      .file(unix_path.join("fs.c"))
      .file(unix_path.join("getaddrinfo.c"))
      .file(unix_path.join("getnameinfo.c"))
      .file(unix_path.join("loop-watcher.c"))
      .file(unix_path.join("loop.c"))
      .file(unix_path.join("pipe.c"))
      .file(unix_path.join("poll.c"))
      .file(unix_path.join("process.c"))
      .file(unix_path.join("random-devurandom.c"))
      .file(unix_path.join("signal.c"))
      .file(unix_path.join("stream.c"))
      .file(unix_path.join("tcp.c"))
      .file(unix_path.join("thread.c"))
      .file(unix_path.join("tty.c"))
      .file(unix_path.join("udp.c"));
  }

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L272,L274
  // if(APPLE OR CMAKE_SYSTEM_NAME MATCHES "Android|Linux")
  //   list(APPEND uv_sources src/unix/proctitle.c)
  // endif()
  if is_apple || is_linux {
    build.file(unix_path.join("proctitle.c"));
  }

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L276,L278
  // if(CMAKE_SYSTEM_NAME MATCHES "DragonFly|FreeBSD")
  //   list(APPEND uv_sources src/unix/is_freebsd.c)
  // endif()
  if is_dragonfly || is_freebsd {
    build.file(unix_path.join("is_freebsd.c"));
  }

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L280,L282
  // if(CMAKE_SYSTEM_NAME MATCHES "DragonFly|FreeBSD|NetBSD|OpenBSD")
  //   list(APPEND uv_sources src/unix/posix-hrtime.c src/unix/bsd-proctitle.c)
  // endif()
  if is_dragonfly || is_freebsd || is_netbsd || is_openbsd {
    build
      .file(unix_path.join("posix-hrtime.c"))
      .file(unix_path.join("bsd-proctitle.c"));
  }

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L284,L286
  // if(APPLE OR CMAKE_SYSTEM_NAME MATCHES "DragonFly|FreeBSD|NetBSD|OpenBSD")
  //   list(APPEND uv_sources src/unix/bsd-ifaddrs.c src/unix/kqueue.c)
  // endif()
  if is_apple || is_dragonfly || is_freebsd || is_netbsd || is_openbsd {
    build
      .file(unix_path.join("bsd-ifaddrs.c"))
      .file(unix_path.join("kqueue.c"));
  }

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L288,L290
  // if(CMAKE_SYSTEM_NAME MATCHES "FreeBSD")
  //   list(APPEND uv_sources src/unix/random-getrandom.c)
  // endif()
  if is_freebsd {
    build.file(unix_path.join("random-getrandom.c"));
  }

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L292,L294
  // if(APPLE OR CMAKE_SYSTEM_NAME STREQUAL "OpenBSD")
  //   list(APPEND uv_sources src/unix/random-getentropy.c)
  // endif()
  if is_apple || is_openbsd {
    build.file(unix_path.join("random-getentropy.c"));
  }

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L296,L302
  // if(APPLE)
  //   list(APPEND uv_defines _DARWIN_UNLIMITED_SELECT=1 _DARWIN_USE_64_BIT_INODE=1)
  //   list(APPEND uv_sources
  //        src/unix/darwin-proctitle.c
  //        src/unix/darwin.c
  //        src/unix/fsevents.c)
  // endif()
  if is_apple {
    build
      .define("_DARWIN_UNLIMITED_SELECT", "1")
      .define("_DARWIN_USE_64_BIT_INODE", "1")
      .file(unix_path.join("darwin-proctitle.c"))
      .file(unix_path.join("darwin.c"))
      .file(unix_path.join("fsevents.c"));
  }

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L304,L313
  // if(CMAKE_SYSTEM_NAME STREQUAL "GNU")
  //   list(APPEND uv_libraries dl)
  //   list(APPEND uv_sources
  //        src/unix/bsd-ifaddrs.c
  //        src/unix/no-fsevents.c
  //        src/unix/no-proctitle.c
  //        src/unix/posix-hrtime.c
  //        src/unix/posix-poll.c
  //        src/unix/hurd.c)
  // endif()
  // TODO: ???

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L315,L323
  // if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
  //   list(APPEND uv_defines _GNU_SOURCE _POSIX_C_SOURCE=200112)
  //   list(APPEND uv_libraries dl rt)
  //   list(APPEND uv_sources
  //        src/unix/is_linux.c
  //        src/unix/procfs-exepath.c
  //        src/unix/random-getrandom.c
  //        src/unix/random-sysctl-is_linux.c)
  // endif()
  if is_linux {
    build
      .define("_GNU_SOURCE", None)
      .define("_POSIX_C_SOURCE", "200112");

    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=rt");
    
    build
      .file(unix_path.join("is_linux.c"))
      .file(unix_path.join("procfs-exepath.c"))
      .file(unix_path.join("random-getrandom.c"))
      .file(unix_path.join("random-sysctl-is_linux.c"));
  }

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L325,L328
  // if(CMAKE_SYSTEM_NAME STREQUAL "NetBSD")
  //   list(APPEND uv_sources src/unix/is_netbsd.c)
  //   list(APPEND uv_libraries kvm)
  // endif()
  if is_netbsd {
    build.file(unix_path.join("is_netbsd.c"));
    println!("cargo:rustc-link-lib=kvm");
  }

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L330,L332
  // if(CMAKE_SYSTEM_NAME STREQUAL "OpenBSD")
  //   list(APPEND uv_sources src/unix/is_openbsd.c)
  // endif()
  if is_openbsd {
    build.file(unix_path.join("is_openbsd.c"));
  }

  // https://github.com/libuv/libuv/blob/v1.45.0/CMakeLists.txt#L389,L399
  // if(CMAKE_SYSTEM_NAME STREQUAL "SunOS")
  //   if(CMAKE_SYSTEM_VERSION STREQUAL "5.10")
  //     list(APPEND uv_defines SUNOS_NO_IFADDRS)
  //     list(APPEND uv_libraries rt)
  //   endif()
  //   list(APPEND uv_defines __EXTENSIONS__ _XOPEN_SOURCE=500 _REENTRANT)
  //   list(APPEND uv_libraries kstat nsl sendfile socket)
  //   list(APPEND uv_sources
  //        src/unix/no-proctitle.c
  //        src/unix/sunos.c)
  // endif()
  if is_solaris {
    build
      .define("__EXTENSIONS__", None)
      .define("_XOPEN_SOURCE", "500")
      .define("_REENTRANT", None);

    println!("cargo:rustc-link-lib=kstat");
    println!("cargo:rustc-link-lib=nsl");
    println!("cargo:rustc-link-lib=sendfile");
    println!("cargo:rustc-link-lib=socket");

    build
      .file(unix_path.join("no-proctitle.c"))
      .file(unix_path.join("sunos.c"));
  }

  build.compile("uv");
}

fn write_bindings(content: &str, path: &str) {
  // write to file
  let filename = PathBuf::from(path).join("bindings.rs");
  let mut file = OpenOptions::new()
    .write(true)
    .truncate(true)
    .create(true)
    .open(filename.clone())
    .unwrap_or_else(|_| panic!("Failed to open {}", filename.to_string_lossy()));

  file.write_all(content.as_bytes())
    .unwrap_or_else(|_| panic!("Failed to write to {}", filename.to_string_lossy()));
}

fn generate_bindings<P: AsRef<Path>>(include_path: &P) {
  let include_path  = include_path.as_ref();
  let header_path   = include_path.join("uv.h");

  let bindings = bindgen::Builder::default()
    // .raw_line("#![allow(non_camel_case_types)]")
    // .raw_line("#![allow(non_snake_case)]")
    // .raw_line("#![allow(non_upper_case_globals)]")
    // .raw_line("#![allow(clippy::missing_safety_doc)]")
    // .raw_line("#![allow(clippy::redundant_static_lifetimes)]")
    .header(header_path.to_string_lossy())
    .clang_arg(format!("-I{}", include_path.display()))
    .derive_debug(true)
    .derive_default(true)
    .prepend_enum_name(false)
    .whitelist_type("uv_.+")
    .whitelist_function("uv_.+")
    .whitelist_var("(?i)uv_.+")
    .whitelist_var("AF_.+")
    .whitelist_var("AI_.+")
    .whitelist_var("IPPROTO_.+")
    .whitelist_var("NI_.+")
    .whitelist_var("SIG.+")
    .whitelist_var("SOCK_.+")
    .whitelist_type("__socket_type.*")
    .whitelist_type("IPPROTO")
    .generate()
    .unwrap_or_else(|_| panic!("Unable to generate bindings"));

  let output = bindings.to_string();

  // On some Linux systems, the SOCK_* constants end up getting prefixed with __socket_type_.
  // Additionally, on Windows, the IPPROTO_* constants get prefixed with IPPROTO_.
  // We'll strip those prefixes here.
  let output = output
    .replace("::std::os::raw::", "")
    .replace("::std::ptr::", "ptr::")
    .replace("::std::mem::", "mem::")
    .replace("::std::option::Option", "Option")
    .replace("__socket_type_", "")
    .replace("IPPROTO_IPPROTO_", "IPPROTO_")
    .replace("#[test]\nfn bindgen_test_layout", "#[test]\n#[allow(deref_nullptr)]\nfn bindgen_test_layout");
    // .replace("#[test]", "#[test]\n#[allow(deref_nullptr)]");
    // .replace("uv_errno_t_UV", "UV")
    // .replace("uv_fs_type_UV", "UV")
    // .replace("uv_req_type_UV", "UV")
    // .replace("uv_run_mode_UV", "UV")
    // .replace("uv_fs_event_UV", "UV")
    // .replace("uv_clock_id_UV", "UV")
    // .replace("uv_tcp_flags_UV", "UV")
    // .replace("uv_udp_flags_UV", "UV")
    // .replace("uv_poll_event_UV", "UV")
    // .replace("uv_membership_UV", "UV")
    // .replace("uv_tty_mode_t_UV", "UV")
    // .replace("uv_stdio_flags_UV", "UV")
    // .replace("uv_handle_type_UV", "UV")
    // .replace("uv_loop_option_UV", "UV")
    // .replace("uv_process_flags_UV", "UV")
    // .replace("uv_dirent_type_t_UV", "UV")
    // .replace("uv_fs_event_flags_UV", "UV")
    // .replace("uv_tty_vtermstate_t_UV", "UV")
    // .replace("uv_thread_create_flags_UV", "UV");

  write_bindings(&output, &env::var("OUT_DIR").unwrap());
}

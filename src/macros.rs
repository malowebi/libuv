#[macro_export]
macro_rules! repr_ {
  ($vis:vis, $T1:ident, $T2:ty) => {
    $vis type $T1 = $T2;

    // impl Drop for $T1 {
    //   fn drop(&mut self) {
    //     // println!("Drop({})", stringify!($T1));
    //   }
    // }

    impl $T1 {
      pub fn boxed() -> Box<Self>  {
        // Box::new(Self::default())
        Box::<$T1>::default()
      }
    
      pub fn as_ptr(&self) -> *const $T1 {
        self as *const $T1
      }
    
      pub fn as_mut_ptr(&mut self) -> *mut $T1 {
        self as *mut $T1
      }
    }
  };
}


#[macro_export]
macro_rules! repr {
  (
    $vis:vis type $T1:ident = $T2:ty;
    $($t:tt)*
  ) => {
    repr_!($vis, $T1, $T2);
    $($t)*
  };
  (
    $vis:vis type $T1:ident = $T2:ty;
  ) => {
    repr_!($vis, $T1, $T2);
  };
}




#[macro_export]
macro_rules! idle {
  ($ptr:ident) => {
    let mut idle = unsafe { Box::from_raw($ptr) };
  };
}

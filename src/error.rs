pub enum ShmemError {
    Open(i32),
    Resize(i32),
    Mmap(i32),
}

#[macro_export]
macro_rules! inspecterr {
    ($val: expr, $variant: ident) => {
        inspecterr!($val, $variant, -1)
    };
    ($val: expr, $variant: ident, $err: expr) => {{
        #[cfg(target_os = "linux")]
        use libc::__errno_location as errno_location;

        #[cfg(target_os = "macos")]
        use libc::__error as errno_location;

        if $val == $err {
            let errcode = unsafe { *errno_location() };
            return Err($crate::error::ShmemError::$variant(errcode));
        }
    }};
}

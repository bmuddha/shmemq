use std::fmt::Display;

/// Possible error codes when working with shared memory queue
#[derive(Debug)]
pub enum ShmemError {
    /// Failed to open provided shared memory file
    Open(i32),
    /// Failed to resize the shared memory file to specified size
    Resize(i32),
    /// Failed to query the size of shared memory file
    SizeCheck(i32),
    /// Failed to memory map the shared memory file
    Mmap(i32),
}

impl std::error::Error for ShmemError {}

impl Display for ShmemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShmemError::Open(code) => {
                write!(f, "Failed to open shared memory: error {code}")
            }
            ShmemError::Resize(code) => {
                write!(f, "Failed to resize shared memory: error {code}")
            }
            ShmemError::SizeCheck(code) => {
                write!(f, "Failed to obtain size of shared memory: error {code}",)
            }
            ShmemError::Mmap(code) => {
                write!(f, "Failed to mmap shared memory: error {code}")
            }
        }
    }
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

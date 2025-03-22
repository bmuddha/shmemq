#[cfg(target_os = "linux")]
use std::ptr::null_mut;

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::*;

#[cfg(target_os = "linux")]
pub(crate) struct Synchronizer<const ROLE: i32> {
    flag: *mut i32,
}
#[cfg(not(target_os = "linux"))]
pub(crate) struct Synchronizer<const ROLE: i32> {
    flag: *mut i32,
    sem: *mut i32,
}

#[repr(i32)]
pub enum Role {
    Producer = i32::MIN,
    Consumer = i32::MAX,
}

impl Role {
    pub const PRODUCER: i32 = Self::Producer as i32;
    pub const CONSUMER: i32 = Self::Consumer as i32;
}

impl<const ROLE: i32> Synchronizer<ROLE> {
    #[cfg(target_os = "linux")]
    pub(crate) fn new(flag: *mut i32) -> Self {
        Self { flag }
    }
    #[cfg(not(target_os = "linux"))]
    pub(crate) fn new(name: &str, flag: *mut i32) -> Self {
        use std::ffi::CString;

        let cstr = CString::new(name).unwrap();
        let id = cstr.as_ptr();
        let sem = unsafe { libc::sem_open(id, libc::O_CREAT, 0o644, 0) };
        if sem == libc::SEM_FAILED {
            panic!("failed to open named semaphore {name}");
        }
        Self { sem, flag }
    }

    pub(crate) fn wait(&self) {
        unsafe { self.wait_inner() };
    }

    pub(crate) fn wake(&self) {
        unsafe { self.wake_inner() };
    }

    #[cfg(target_os = "linux")]
    #[inline(always)]
    unsafe fn wait_inner(&self) {
        libc::syscall(
            libc::SYS_futex,
            self.flag,
            libc::FUTEX_WAIT,
            0,
            null_mut::<u32>(),
            null_mut::<u32>(),
            0,
        );
        let atomic = self.flag as *const AtomicI32;
        unsafe { &*atomic }.fetch_sub(1, Release);
    }

    #[cfg(target_os = "linux")]
    #[inline(always)]
    unsafe fn wake_inner(&self) {
        let atomic = self.flag as *const AtomicI32;
        unsafe { &*atomic }.fetch_add(1, Release);
        libc::syscall(
            libc::SYS_futex,
            self.flag,
            libc::FUTEX_WAKE,
            1, // wake one thread
            null_mut::<u32>(),
            null_mut::<u32>(),
            0,
        );
    }

    #[cfg(not(target_os = "linux"))]
    #[inline(always)]
    unsafe fn wait_inner(&self) {
        libc::sem_wait(self.sem);
    }

    #[cfg(not(target_os = "linux"))]
    #[inline(always)]
    unsafe fn wake_inner(&self) {
        libc::sem_post(self.sem);
    }
}

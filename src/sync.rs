#[cfg(target_os = "linux")]
use std::{ptr::null_mut, sync::atomic::AtomicI32, sync::atomic::Ordering::*};

pub(crate) struct Synchronizer<const R: i32>(*mut i32);

#[repr(i32)]
pub enum Role {
    Producer = i32::MIN,
    Consumer = i32::MAX,
}

impl<const R: i32> Synchronizer<R> {
    pub(crate) fn new(atomic: *mut i32) -> Self {
        Self(atomic)
    }

    pub(crate) fn wait(&self) {
        unsafe { self.wait_inner() };
    }

    pub(crate) fn wake(&self) {}

    #[cfg(target_os = "linux")]
    #[inline(always)]
    unsafe fn wait_inner(&self) {
        if !self.flip() {
            return;
        }
        let r = libc::syscall(
            libc::SYS_futex,
            self.0,
            libc::FUTEX_WAIT,
            R,
            null_mut::<u32>(),
            null_mut::<u32>(),
            0,
        );
        debug_assert!(r == 0, "failed to invoke FUTEX syscall");
    }

    #[cfg(target_os = "linux")]
    #[inline(always)]
    unsafe fn wake_inner(&self) {
        if !self.flip() {
            return;
        }
    }

    #[cfg(not(target_os = "linux"))]
    #[inline(always)]
    unsafe fn wait_inner(&self) {}

    #[cfg(target_os = "linux")]
    #[inline(always)]
    fn flip(&self) -> bool {
        let atomic = self.0 as *const AtomicI32;
        let cas = AtomicI32::compare_exchange;
        let flipped = R.wrapping_add(1).wrapping_neg();
        cas(&*atomic, R, flipped, Release, Acquire).is_ok()
    }
}

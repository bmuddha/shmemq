#[cfg(target_os = "linux")]
use std::{ptr::null_mut, sync::atomic::AtomicI32, sync::atomic::Ordering::*};

pub(crate) struct Synchronizer<const ROLE: i32>(*mut i32);

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
        let atomic = self.0 as *const AtomicI32;
        let expected = Self::flip_wait_bits();
        unsafe { &*atomic }.store(expected, Release);
        libc::syscall(
            libc::SYS_futex,
            self.0,
            libc::FUTEX_WAIT,
            expected,
            null_mut::<u32>(),
            null_mut::<u32>(),
            0,
        );
    }

    #[cfg(target_os = "linux")]
    #[inline(always)]
    unsafe fn wake_inner(&self) {
        let atomic = self.0 as *const AtomicI32;
        let current = unsafe { &*atomic }.load(Acquire);

        // if no one is waiting for this
        // side of queue, then do nothing
        if current != ROLE {
            return;
        }
        let flipped = Self::flip_wait_bits();
        unsafe { &*atomic }.store(flipped, Release);
        libc::syscall(
            libc::SYS_futex,
            self.0,
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
        libc::sem_wait(self.0);
    }

    #[cfg(not(target_os = "linux"))]
    #[inline(always)]
    unsafe fn wake_inner(&self) {
        libc::sem_post(self.0);
    }

    #[cfg(target_os = "linux")]
    #[inline(always)]
    fn flip_wait_bits() -> i32 {
        ROLE.wrapping_add(1).wrapping_neg()
    }
}

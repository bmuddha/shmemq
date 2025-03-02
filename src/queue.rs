use std::{ptr::null_mut, sync::atomic::AtomicU64};

use crate::{inspecterr, ShmemResult, ShmemSettings, DOUBLE_WORD};

pub(crate) struct ShmemQueue<T: Copy> {
    base: *const T,
    offsets: *const AtomicU64,
}

impl<T: Copy> ShmemQueue<T> {
    /// # Safety
    ///
    pub(crate) unsafe fn new(settings: ShmemSettings) -> ShmemResult<Self> {
        let oflag = libc::O_CREAT | libc::O_RDWR;
        let name = settings.name.as_ptr() as *const libc::c_char;
        #[cfg(target_os = "linux")]
        let fd = libc::shm_open(name, oflag, 0o644);
        #[cfg(not(target_os = "linux"))]
        let fd = libc::shm_open(name, oflag);

        inspecterr!(fd, Open);

        let length = settings.size * size_of::<T>() + DOUBLE_WORD;
        let code = libc::ftruncate(fd, length as libc::off_t);

        inspecterr!(code, Resize);

        let addr = libc::mmap(
            null_mut(),
            length,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        );
        inspecterr!(addr, Mmap, libc::MAP_FAILED);

        let addr = addr as *mut u8;
        let base = addr.add(DOUBLE_WORD);
        let base = base.add(base.align_offset(align_of::<T>())) as *const T;
        let offsets = addr as *const AtomicU64;

        Ok(Self { base, offsets })
    }

    pub(crate) fn syncword(&self) -> *mut i32 {
        unsafe { self.offsets.add(1) as *mut i32 }
    }

    pub(crate) fn pop(&self) -> T {
        todo!()
    }

    pub(crate) fn push(&self, item: T) {
        todo!()
    }
}

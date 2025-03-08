use std::{ptr::null_mut, sync::atomic::AtomicU32};

use crate::{inspecterr, ShmemResult, ShmemSettings, WORD};

pub(crate) struct ShmemQueue<T: Copy> {
    base: *mut T,
    pub(crate) capacity: u32,
    offset: u32,
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

        let length = settings.size * size_of::<T>() + WORD;
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
        let base = addr.add(WORD);
        let base = base.add(base.align_offset(align_of::<T>())) as *mut T;

        Ok(Self {
            base,
            offset: 0,
            capacity: settings.size as u32,
        })
    }

    pub(crate) fn syncword(&self) -> *mut i32 {
        unsafe { self.base.sub(1) as *mut i32 }
    }

    pub(crate) fn length(&self) -> *const AtomicU32 {
        unsafe { (self.base as *const AtomicU32).sub(1) }
    }

    pub(crate) unsafe fn read(&mut self) -> T {
        let val = self.base.add(self.offset as usize).read();
        self.offset += 1;
        self.offset %= self.capacity;
        val
    }

    pub(crate) unsafe fn write(&mut self, val: T) {
        self.base.add(self.offset as usize).write(val);
        self.offset += 1;
        self.offset %= self.capacity;
    }
}

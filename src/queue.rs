use std::{ffi::CString, ptr::null_mut, sync::atomic::AtomicU32};

use crate::{inspecterr, ShmemResult, ShmemSettings, METASIZE};

pub(crate) struct ShmemQueue<T: Copy> {
    base: *mut T,
    pub(crate) capacity: u32,
    offset: u32,
}

impl<T: Copy> ShmemQueue<T> {
    /// # Safety
    ///
    pub(crate) unsafe fn new(settings: &ShmemSettings) -> ShmemResult<Self> {
        let oflag = libc::O_CREAT | libc::O_RDWR;
        let cstr = CString::new(settings.name.as_str()).unwrap();
        let name = cstr.as_ptr();
        let fd = libc::shm_open(name, oflag, 0o644);

        inspecterr!(fd, Open);

        let empty = {
            let mut stats: libc::stat = unsafe { std::mem::zeroed() };
            let code = libc::fstat(fd, &mut stats);
            inspecterr!(code, SizeCheck);
            stats.st_size == 0
        };

        let length = settings.size * size_of::<T>() + METASIZE;

        if empty {
            let code = libc::ftruncate(fd, length as libc::off_t);
            inspecterr!(code, Resize);
        }

        let addr = libc::mmap(
            null_mut(),
            length,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        );
        inspecterr!(addr, Mmap, libc::MAP_FAILED);

        (addr as *mut i32).write(0);
        (addr as *mut i32).add(1).write(0);
        let addr = addr as *mut u8;
        let base = addr.add(METASIZE);
        let base = base.add(base.align_offset(align_of::<T>())) as *mut T;

        Ok(Self {
            base,
            offset: 0,
            capacity: settings.size as u32,
        })
    }

    pub(crate) fn syncword(&self) -> *mut i32 {
        unsafe { (self.base as *mut u32).sub(2) as *mut i32 }
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

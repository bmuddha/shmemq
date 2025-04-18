use std::{ffi::CString, ptr::null_mut};

use crate::{inspecterr, ShmemResult, ShmemSettings, METASIZE};
pub(crate) const LEN_PREFIX_SIZE: u32 = std::mem::size_of::<u32>() as u32;

/// The core type for managing shared memory queue
pub(crate) struct ShmemQueue<T: Copy> {
    /// starting address of the queue
    base: *mut T,
    /// total capacity (in terms of size of T) of the queue
    pub(crate) capacity: u32,
    /// current offset (in terms of T) into the queue,
    /// where next element is to be read from
    offset: u32,
    /// name of the shared memory file
    name: CString,
}

impl<T: Copy> ShmemQueue<T> {
    /// # Safety
    /// Caller must make sure that there's only one consumer and only one
    /// producer endpont, otherwise it will result in undefined behavior
    pub(crate) unsafe fn new(settings: &ShmemSettings) -> ShmemResult<Self> {
        let oflag = libc::O_CREAT | libc::O_RDWR;
        let name = CString::new(settings.name.as_str()).unwrap();
        let fd = libc::shm_open(name.as_ptr(), oflag, 0o644);

        inspecterr!(fd, Open);

        let empty = {
            let mut stats: libc::stat = std::mem::zeroed();
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
            name,
        })
    }

    unsafe fn pointer(&self) -> *mut T {
        self.base.add(self.offset as usize)
    }

    pub(crate) fn syncword(&self) -> *mut i32 {
        unsafe { (self.base as *mut u32).sub(2) as *mut i32 }
    }

    pub(crate) unsafe fn read(&mut self) -> T {
        let val = self.pointer().read();
        self.offset += 1;
        self.offset %= self.capacity;
        val
    }

    pub(crate) unsafe fn write(&mut self, val: T) {
        self.pointer().write(val);
        self.offset += 1;
        self.offset %= self.capacity;
    }
}

impl ShmemQueue<u8> {
    pub(crate) unsafe fn read_slice(&mut self) -> &[u8] {
        if self.offset == self.capacity {
            self.offset = 0;
        }
        let mut len = (self.pointer() as *const u32).read();
        if len == u32::MAX {
            self.offset = 0;
            len = (self.pointer() as *const u32).read();
        }
        self.offset += LEN_PREFIX_SIZE;
        let ptr = self.pointer();
        self.offset += roundup_to_u32_align(len);
        std::slice::from_raw_parts(ptr, len as usize)
    }

    pub(crate) unsafe fn write_slice(&mut self, slice: &[u8]) {
        let remaining = self.capacity - self.offset;
        let len = slice.len() as u32;
        let rounded_len = roundup_to_u32_align(len);

        if remaining == 0 {
            self.offset = 0;
        } else if remaining < rounded_len + LEN_PREFIX_SIZE {
            (self.pointer() as *mut u32).write(u32::MAX);
            self.offset = 0;
        }
        (self.pointer() as *mut u32).write(len);
        self.offset += LEN_PREFIX_SIZE;
        self.pointer()
            .copy_from_nonoverlapping(slice.as_ptr(), slice.len());
        self.offset += rounded_len;
    }
}

impl<T: Copy> Drop for ShmemQueue<T> {
    fn drop(&mut self) {
        unsafe { libc::shm_unlink(self.name.as_ptr()) };
    }
}

#[inline(always)]
pub(crate) fn roundup_to_u32_align(val: u32) -> u32 {
    val + (LEN_PREFIX_SIZE - val % LEN_PREFIX_SIZE) % LEN_PREFIX_SIZE
}

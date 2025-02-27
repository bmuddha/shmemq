use core::sync::atomic::AtomicU64;
use std::ptr::null_mut;

use error::ShmemError;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ShmemSettings {
    pub name: String,
    pub size: usize,
}

const DOUBLE_WORD: usize = size_of::<usize>() * 2;

pub struct ShmemQueue<T: Sized + Copy> {
    base: *const T,
    offsets: *const AtomicU64,
}

impl<T: Sized + Copy> ShmemQueue<T> {
    /// # Safety
    ///
    pub unsafe fn new(settings: ShmemSettings) -> Result<Self, ShmemError> {
        let oflag = libc::O_CREAT | libc::O_RDWR;
        let name = settings.name.as_ptr() as *const libc::c_char;
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
}

mod consumer;
mod error;
mod producer;

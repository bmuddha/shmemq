use core::sync::atomic::AtomicU64;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ShmemSettings {
    pub name: String,
    pub size: usize,
}

pub struct ShmemQueue<T: Sized + Copy> {
    base: *const T,
    offsets: *const AtomicU64,
}

impl<T: Sized + Copy> ShmemQueue<T> {
    pub fn new(settings: ShmemSettings) -> Self {
        let oflag = libc::O_CREAT | libc::O_RDWR;
        let name = settings.name.as_ptr() as *const libc::c_char;
        let fd = unsafe { libc::shm_open(name, oflag) };
        todo!()
    }
}

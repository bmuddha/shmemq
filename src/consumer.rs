use crate::{queue::ShmemQueue, ShmemResult, ShmemSettings};

pub struct ShmemConsumer<T: Copy> {
    shm: ShmemQueue<T>,
}

impl<T: Copy> ShmemConsumer<T> {
    pub fn new(settings: ShmemSettings) -> ShmemResult<Self> {
        let shm = unsafe { ShmemQueue::new(settings) }?;
        Ok(Self { shm })
    }

    pub fn consume(&self) -> T {
        todo!()
    }
}

use crate::{queue::ShmemQueue, ShmemResult, ShmemSettings};

pub struct ShmemProducer<T: Copy> {
    shm: ShmemQueue<T>,
}

impl<T: Copy> ShmemProducer<T> {
    pub fn new(settings: ShmemSettings) -> ShmemResult<Self> {
        let shm = unsafe { ShmemQueue::new(settings) }?;
        Ok(Self { shm })
    }

    pub fn produce(&self) -> T {
        todo!()
    }
}

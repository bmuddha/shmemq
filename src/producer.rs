use crate::{
    queue::ShmemQueue,
    sync::{Role, Synchronizer},
    ShmemResult, ShmemSettings,
};

pub struct ShmemProducer<T: Copy> {
    shm: ShmemQueue<T>,
    sync: Synchronizer<{ Role::PRODUCER }>,
}

impl<T: Copy> ShmemProducer<T> {
    pub fn new(settings: ShmemSettings) -> ShmemResult<Self> {
        let shm = unsafe { ShmemQueue::new(settings) }?;
        let sync = Synchronizer::new(shm.syncword());
        Ok(Self { shm, sync })
    }

    pub fn produce(&self) -> T {
        todo!()
    }
}

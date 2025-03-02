use crate::{
    queue::ShmemQueue,
    sync::{Role, Synchronizer},
    ShmemResult, ShmemSettings,
};

pub struct ShmemConsumer<T: Copy> {
    shm: ShmemQueue<T>,
    sync: Synchronizer<{ Role::CONSUMER }>,
}

impl<T: Copy> ShmemConsumer<T> {
    pub fn new(settings: ShmemSettings) -> ShmemResult<Self> {
        let shm = unsafe { ShmemQueue::new(settings) }?;
        let sync = Synchronizer::new(shm.syncword());
        Ok(Self { shm, sync })
    }

    pub fn consume(&self) -> T {
        todo!()
    }
}

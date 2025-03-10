use std::sync::atomic::AtomicU32;

use error::ShmemError;
use queue::ShmemQueue;
use sync::Synchronizer;

pub type ShmemResult<T> = Result<T, ShmemError>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ShmemSettings {
    pub name: String,
    pub size: usize,
}

const METASIZE: usize = size_of::<u64>();

pub struct ShmemEndpoint<T: Copy, const ROLE: i32> {
    shm: ShmemQueue<T>,
    sync: Synchronizer<ROLE>,
    length: *const AtomicU32,
}

unsafe impl<T: Copy, const ROLE: i32> Send for ShmemEndpoint<T, ROLE> {}

impl<T: Copy, const ROLE: i32> ShmemEndpoint<T, ROLE> {
    pub fn new(settings: ShmemSettings) -> ShmemResult<Self> {
        let shm = unsafe { ShmemQueue::new(&settings) }?;

        #[cfg(target_os = "linux")]
        let sync = Synchronizer::new(shm.syncword());
        #[cfg(not(target_os = "linux"))]
        let sync = Synchronizer::new(&settings.name, shm.syncword());

        let length = shm.length();
        Ok(Self { shm, sync, length })
    }

    #[inline(always)]
    fn length(&self) -> &AtomicU32 {
        unsafe { &*self.length }
    }
}

mod consumer;
mod error;
mod producer;
mod queue;
mod sync;
#[cfg(test)]
mod tests;

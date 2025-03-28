use std::sync::atomic::Ordering;

use error::ShmemError;
use queue::ShmemQueue;
use sync::Synchronizer;

pub type ShmemResult<T> = Result<T, ShmemError>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ShmemSettings {
    /// name of file (used for memory mapping), should start with '/'
    pub name: String,
    /// size of queue in terms of the type size
    /// which will be used as an element of queue
    pub size: usize,
}

const METASIZE: usize = size_of::<u64>();

pub struct ShmemEndpoint<T: Copy, const ROLE: i32> {
    shm: ShmemQueue<T>,
    sync: Synchronizer<ROLE>,
}

unsafe impl<T: Copy, const ROLE: i32> Send for ShmemEndpoint<T, ROLE> {}

impl<T: Copy, const ROLE: i32> ShmemEndpoint<T, ROLE> {
    pub fn new(settings: ShmemSettings) -> ShmemResult<Self> {
        let shm = unsafe { ShmemQueue::new(&settings) }?;

        #[cfg(target_os = "linux")]
        let sync = Synchronizer::new(shm.syncword());
        #[cfg(not(target_os = "linux"))]
        let sync = Synchronizer::new(&settings.name, shm.syncword());

        Ok(Self { shm, sync })
    }

    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.sync.inner().load(Ordering::Acquire) == self.shm.capacity as i32
    }
}

pub mod consumer;
mod error;
pub mod producer;
mod queue;
mod sync;
#[cfg(test)]
mod tests;

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

const WORD: usize = size_of::<usize>();

pub struct ShmemEndpoint<T: Copy, const ROLE: i32> {
    shm: ShmemQueue<T>,
    sync: Synchronizer<ROLE>,
    length: *const AtomicU32,
}

impl<T: Copy, const ROLE: i32> ShmemEndpoint<T, ROLE> {
    pub fn new(settings: ShmemSettings) -> ShmemResult<Self> {
        let shm = unsafe { ShmemQueue::new(settings) }?;
        let sync = Synchronizer::new(shm.syncword());
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

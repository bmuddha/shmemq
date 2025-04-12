//! Shared memory queue library for inter process communication. The library provides a single
//! producer single consumer channel. Although it can be used for inter-thread communication within
//! the same process, other alternatives like crossbeam or flume channels offer more than twice as
//! much throughput and lower latency. The main interface for using the library is [ShmemEndpoint]
//! type which has two concrete implementation [ShmemProducer] and [ShmemConsumer]. The channel is
//! generic over any type that is a Copy.

use std::sync::atomic::Ordering;

use error::ShmemError;
use queue::ShmemQueue;
use sync::Synchronizer;

pub use consumer::ShmemConsumer;
pub use producer::ShmemProducer;

pub type ShmemResult<T> = Result<T, ShmemError>;

/// Settings for shared memory queue
/// _Note_: it should be identical for consumer and producer processes
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ShmemSettings {
    /// name of file (used for memory mapping), should start with '/' prefix
    pub name: String,
    /// size of queue in terms of the type size
    /// which will be used as an element of queue
    pub size: usize,
}

const METASIZE: usize = size_of::<u64>();

/// One of the endpoints of shared memory queue, depending on the ROLE constant can be either a
/// consumer or producer endpoint. It's more convenient to use [ShmemConsumer] or [ShmemProducer]
/// directly.
///
/// # Safety
/// User must make sure that there's only one consumer and only one producer, otherwise it will
/// result in undefined behavior
pub struct ShmemEndpoint<T: Copy, const ROLE: i8> {
    shm: ShmemQueue<T>,
    sync: Synchronizer,
}

unsafe impl<T: Copy, const ROLE: i8> Send for ShmemEndpoint<T, ROLE> {}

impl<T: Copy, const ROLE: i8> ShmemEndpoint<T, ROLE> {
    /// Opens the shared memory queue endpont
    ///
    /// # Safety
    /// Caller must make sure that there's only one consumer and only one
    /// producer endpont, otherwise it will result in undefined behavior
    pub fn new(settings: ShmemSettings) -> ShmemResult<Self> {
        let shm = unsafe { ShmemQueue::new(&settings) }?;

        #[cfg(target_os = "linux")]
        let sync = Synchronizer::new(shm.syncword());
        #[cfg(not(target_os = "linux"))]
        let sync = Synchronizer::new(&settings.name, shm.syncword());

        Ok(Self { shm, sync })
    }

    /// Checks whether the queue capacity has been exhausted
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.sync.inner().load(Ordering::Relaxed) == self.shm.capacity
    }
}

impl<const ROLE: i8> ShmemEndpoint<u8, ROLE> {
    #[inline]
    pub(crate) fn decrement_count(&self, amount: u32) {
        #[cfg(target_os = "linux")]
        let amount = amount - 1;
        self.sync.inner().fetch_sub(amount, Ordering::Relaxed);
    }

    #[inline]
    pub(crate) fn increment_count(&self, amount: u32) {
        #[cfg(target_os = "linux")]
        let amount = amount - 1;
        self.sync.inner().fetch_add(amount, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn has_capacity(&self, extra: u32) -> bool {
        self.sync.inner().load(Ordering::Relaxed) + extra >= self.shm.capacity
    }
}

pub mod consumer;
mod error;
pub mod producer;
mod queue;
mod sync;
#[cfg(test)]
mod tests;

use crate::{
    queue::{roundup_to_u32_align, LEN_PREFIX_SIZE},
    sync::Role,
    ShmemEndpoint,
};

pub type ShmemProducer<T> = ShmemEndpoint<T, { Role::PRODUCER }>;

impl<T: Copy> ShmemEndpoint<T, { Role::PRODUCER }> {
    /// Produce/Send a message to the channel
    /// _Note_: This method will busy block if the channel is full
    /// Thus it's not recommended to run this on systems where
    /// consumers are consistently slower than producers
    pub fn produce(&mut self, val: T) {
        while self.is_full() {
            std::thread::yield_now();
        }
        unsafe { self.shm.write(val) };
        self.sync.wake();
    }
}

impl ShmemEndpoint<u8, { Role::PRODUCER }> {
    /// Produce/Send a message to the channel
    /// _Note_: This method will busy block if the channel is full
    /// Thus it's not recommended to run this on systems where
    /// consumers are consistently slower than producers
    pub fn produce_slice(&mut self, slice: impl AsRef<[u8]>) {
        let slice = slice.as_ref();
        let amount = roundup_to_u32_align(slice.len() as u32) + LEN_PREFIX_SIZE;
        while self.has_capacity(amount) {
            std::thread::yield_now();
        }
        unsafe { self.shm.write_slice(slice) };
        self.increment_count(amount);
        self.sync.wake();
    }
}

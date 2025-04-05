use crate::{sync::Role, ShmemEndpoint};

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

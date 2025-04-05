use crate::{sync::Role, ShmemEndpoint};

pub type ShmemConsumer<T> = ShmemEndpoint<T, { Role::CONSUMER }>;

impl<T: Copy> ShmemEndpoint<T, { Role::CONSUMER }> {
    /// Consume/Receive a message from the channel
    /// This method will block if the channel is empty
    pub fn consume(&mut self) -> T {
        self.sync.wait();
        unsafe { self.shm.read() }
    }
}

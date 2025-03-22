use crate::{sync::Role, ShmemEndpoint};

pub type ShmemConsumer<T> = ShmemEndpoint<T, { Role::CONSUMER }>;

impl<T: Copy> ShmemEndpoint<T, { Role::CONSUMER }> {
    pub fn consume(&mut self) -> T {
        self.sync.wait();
        unsafe { self.shm.read() }
    }
}

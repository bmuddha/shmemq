use crate::{sync::Role, ShmemEndpoint};

pub type ShmemProducer<T> = ShmemEndpoint<T, { Role::PRODUCER }>;

impl<T: Copy> ShmemEndpoint<T, { Role::PRODUCER }> {
    pub fn produce(&mut self, val: T) {
        while self.is_full() {
            std::thread::yield_now();
        }
        unsafe { self.shm.write(val) };
        self.sync.wake();
    }
}

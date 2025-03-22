use std::sync::atomic::Ordering::*;

use crate::{sync::Role, ShmemEndpoint};

pub type ShmemProducer<T> = ShmemEndpoint<T, { Role::PRODUCER }>;

impl<T: Copy> ShmemEndpoint<T, { Role::PRODUCER }> {
    pub fn produce(&mut self, val: T) {
        while self.shm.capacity == self.length().load(Acquire) {
            std::thread::yield_now();
        }
        unsafe { self.shm.write(val) };
        self.length().fetch_add(1, Release);
        self.sync.wake();
    }
}

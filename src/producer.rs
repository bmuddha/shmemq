use std::sync::atomic::Ordering::*;

use crate::{sync::Role, ShmemEndpoint};

pub type ShmemProducer<T> = ShmemEndpoint<T, { Role::PRODUCER }>;

impl<T: Copy> ShmemEndpoint<T, { Role::PRODUCER }> {
    pub fn produce(&mut self, val: T) {
        if self.shm.capacity == self.length().load(Acquire) {
            self.sync.wait();
        }
        unsafe { self.shm.write(val) };
        self.length().fetch_add(1, Release);
        self.sync.wake();
    }
}

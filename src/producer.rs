use std::sync::atomic::Ordering::*;

use crate::{sync::Role, ShmemEndpoint};

pub type ShmemProducer<T> = ShmemEndpoint<T, { Role::PRODUCER }>;

impl<T: Copy> ShmemProducer<T> {
    pub fn produce(&mut self, val: T) {
        if self.shm.capacity == self.length().load(Acquire) {
            self.sync.wait();
        }
        unsafe { self.shm.write(val) };
        let prev = self.length().fetch_add(1, Release);
        if prev == 0 {
            self.sync.wake();
        }
    }
}

use std::sync::atomic::Ordering::*;

use crate::{sync::Role, ShmemEndpoint};

pub type ShmemConsumer<T> = ShmemEndpoint<T, { Role::CONSUMER }>;

impl<T: Copy> ShmemConsumer<T> {
    pub fn consume(&mut self) -> T {
        if self.length().load(Acquire) == 0 {
            self.sync.wait();
        }
        let val = unsafe { self.shm.read() };
        let prev = self.length().fetch_sub(1, Release);
        if prev == self.shm.capacity {
            self.sync.wake();
        }
        val
    }
}

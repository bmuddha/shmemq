use std::sync::atomic::Ordering::*;

use crate::{sync::Role, ShmemEndpoint};

pub type ShmemConsumer<T> = ShmemEndpoint<T, { Role::CONSUMER }>;

impl<T: Copy> ShmemEndpoint<T, { Role::CONSUMER }> {
    pub fn consume(&mut self) -> T {
        while self.length().load(Acquire) == 0 {
            self.sync.wait();
        }
        self.length().fetch_sub(1, Release);
        unsafe { self.shm.read() }
    }
}

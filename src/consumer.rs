use std::ops::Deref;

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

impl ShmemEndpoint<u8, { Role::CONSUMER }> {
    pub fn consume_slice(&mut self) -> SliceGuard {
        self.sync.wait();
        let slice = unsafe { self.shm.read_slice() };
        let slice = unsafe { std::mem::transmute::<&'_ [u8], &'_ [u8]>(slice) };
        SliceGuard {
            slice,
            endpoint: self,
        }
    }
}

pub struct SliceGuard<'shm> {
    slice: &'shm [u8],
    endpoint: &'shm mut ShmemEndpoint<u8, { Role::CONSUMER }>,
}

impl Deref for SliceGuard<'_> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.slice
    }
}

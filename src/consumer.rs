use std::ops::Deref;

use crate::{
    queue::{roundup_to_u32_align, LEN_PREFIX_SIZE},
    sync::Role,
    ShmemEndpoint,
};

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

impl Drop for SliceGuard<'_> {
    fn drop(&mut self) {
        let amount = roundup_to_u32_align(self.slice.len() as u32) + LEN_PREFIX_SIZE;
        self.endpoint.decrement_count(amount);
    }
}

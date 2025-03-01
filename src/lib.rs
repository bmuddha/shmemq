use error::ShmemError;

pub type ShmemResult<T> = Result<T, ShmemError>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ShmemSettings {
    pub name: String,
    pub size: usize,
}

const DOUBLE_WORD: usize = size_of::<usize>() * 2;

mod consumer;
mod error;
mod producer;
mod queue;
mod sync;

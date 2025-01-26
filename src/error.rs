use ps_alloc::{AllocationError, DeallocationError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum BufferError {
    #[error(transparent)]
    AllocationError(#[from] AllocationError),
    #[error(transparent)]
    DeallocationError(#[from] DeallocationError),
}

use ps_alloc::{AllocationError, DeallocationError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BufferError {
    #[error(transparent)]
    AllocationError(#[from] AllocationError),
    #[error(transparent)]
    DeallocationError(#[from] DeallocationError),
}

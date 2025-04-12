use crate::{Buffer, BufferError};

impl Buffer {
    #[inline]
    #[allow(clippy::should_implement_trait)]
    /// # Errors
    /// - `AllocationError` is returned if allocation fails.
    /// - `DeallocationError` is returned if deallocation fails.
    pub fn clone(&self) -> Result<Self, BufferError> {
        Self::from_slice(&self[..])
    }
}

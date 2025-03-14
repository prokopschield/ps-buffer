use crate::{Buffer, BufferError};

impl Buffer {
    /// # Errors
    /// - `AllocationError` is returned if allocation fails.
    /// - `DeallocationError` is returned if deallocation fails.
    pub fn with_capacity(capacity: usize) -> Result<Self, BufferError> {
        let mut buffer = Self::new();

        buffer.reserve_total(capacity)?;

        Ok(buffer)
    }
}

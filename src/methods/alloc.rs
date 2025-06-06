use crate::{constants::FILLER, Buffer, BufferError};

impl Buffer {
    #[inline]
    /// Allocates a `Buffer` and initializes its content.
    /// # Errors
    /// `AllocationError` is returned if allocation fails.
    pub fn alloc(length: usize) -> Result<Self, BufferError> {
        let mut buffer = Self::default();

        buffer.resize(length, FILLER)?;

        Ok(buffer)
    }
}

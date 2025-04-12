use crate::{Buffer, BufferError};

impl Buffer {
    /// Allocates a `Buffer` with provided content.
    /// # Errors
    /// `AllocationError` is returned if allocation fails.
    pub fn from_slice<T>(value: T) -> Result<Self, BufferError>
    where
        T: AsRef<[u8]>,
    {
        let mut buffer = Self::default();

        buffer.extend_from_slice(value.as_ref())?;

        Ok(buffer)
    }
}

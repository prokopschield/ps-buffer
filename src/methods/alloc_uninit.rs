use crate::{Buffer, BufferError};

impl Buffer {
    #[inline]
    /// Allocates a `Buffer` and does not initialize its content.
    pub fn alloc_uninit(length: usize) -> Result<Self, BufferError> {
        let mut buffer = Self::with_capacity(length)?;

        buffer.set_len(length)?;

        Ok(buffer)
    }
}

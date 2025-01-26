use crate::{Buffer, BufferError};

impl Buffer {
    pub fn with_capacity(capacity: usize) -> Result<Self, BufferError> {
        let mut buffer = Buffer::default();

        buffer.reserve_total(capacity)?;

        Ok(buffer)
    }
}

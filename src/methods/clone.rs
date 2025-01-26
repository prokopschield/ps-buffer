use crate::{Buffer, BufferError};

impl Buffer {
    pub fn clone(&self) -> Result<Self, BufferError> {
        Buffer::from(&self[..])
    }
}

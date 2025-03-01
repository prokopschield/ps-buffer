use crate::{Buffer, BufferError};

impl Buffer {
    pub fn from<T>(value: T) -> Result<Self, BufferError>
    where
        T: AsRef<[u8]>,
    {
        let mut buffer = Self::default();

        buffer.extend_from_slice(value.as_ref())?;

        Ok(buffer)
    }
}

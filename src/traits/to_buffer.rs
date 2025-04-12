use crate::{Buffer, BufferError};

pub trait ToBuffer {
    /// Creates a new [`Buffer`] containing the byte representation of `self`.
    /// # Errors
    /// - Returns [`BufferError`] if the allocation fails.
    fn to_buffer(&self) -> Result<Buffer, BufferError>;
}

impl<T: AsRef<[u8]>> ToBuffer for T {
    fn to_buffer(&self) -> Result<Buffer, BufferError> {
        Buffer::from_slice(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::BufferError;

    use super::ToBuffer;

    #[test]
    fn from_str() -> Result<(), BufferError> {
        let str = "Hello, world!";
        let buffer = str.to_buffer()?;

        assert_eq!(str.as_bytes(), &buffer[..], "Values should be equal.");

        Ok(())
    }
}

use crate::{Buffer, BufferError, SharedBuffer};

pub trait ToSharedBuffer {
    /// Creates a new [`SharedBuffer`] containing the byte representation of `self`.
    /// # Errors
    /// - Returns [`BufferError`] if the allocation fails.
    fn to_shared_buffer(self) -> Result<SharedBuffer, BufferError>;
}

impl<T: AsRef<[u8]>> ToSharedBuffer for T {
    fn to_shared_buffer(self) -> Result<SharedBuffer, BufferError> {
        Ok(Buffer::from_slice(self)?.share())
    }
}

#[cfg(test)]
mod tests {
    use crate::BufferError;

    use super::ToSharedBuffer;

    #[test]
    fn from_str() -> Result<(), BufferError> {
        let str = "Hello, world!";
        let buffer = str.to_shared_buffer()?;

        assert_eq!(str.as_bytes(), &buffer[..], "Values should be equal.");

        Ok(())
    }
}

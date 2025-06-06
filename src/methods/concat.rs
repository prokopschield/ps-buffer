use crate::{Buffer, BufferError};

impl Buffer {
    /// Returns a new [`Buffer`] which is the result of concatenating all the slices in the list together.
    ///
    /// If the list has no items, then a new zero-length [`Buffer`] is returned.
    ///
    /// # Errors
    /// - [`BufferError::AllocationError`] is returned if allocation fails.
    pub fn concat(list: &[&[u8]]) -> Result<Self, BufferError> {
        let mut length = 0;

        for item in list {
            length += item.len();
        }

        let mut buffer = Self::with_capacity(length)?;

        for item in list {
            buffer.extend_from_slice(item)?;
        }

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Buffer, BufferError};

    #[test]
    fn try_concat() -> Result<(), BufferError> {
        let buffer = Buffer::concat(&[b"Hello,", b" ", b"world", b"!"])?;

        assert_eq!(&buffer.slice(..), b"Hello, world!");

        Ok(())
    }
}

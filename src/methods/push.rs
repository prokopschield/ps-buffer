use crate::{Buffer, BufferError};

impl Buffer {
    /// Appends a single byte to the buffer, automatically resizing if necessary.
    ///
    /// This method adds a `u8` value to the end of the buffer.
    /// If the buffer is at capacity, it attempts to reserve additional space
    /// before inserting the value.
    ///
    /// # Arguments
    ///
    /// * `value` - A value that can be converted into a `u8`
    ///
    /// # Errors
    ///
    /// * `BufferError::Overflow` - If incrementing the length would overflow `usize`
    /// * `BufferError` - If memory allocation fails during resize
    ///
    /// # Examples
    ///
    /// ```
    /// # use ps_buffer::{Buffer, BufferError};
    /// let mut buffer = Buffer::new();
    /// buffer.push(42)?;
    /// assert_eq!(buffer.slice(..), &[42]);
    /// buffer.push(65)?;
    /// assert_eq!(buffer.slice(..), &[42, 65]);
    /// # Ok::<(), BufferError>(())
    /// ```
    pub fn push(&mut self, value: impl Into<u8>) -> Result<&mut Self, BufferError> {
        if self.len() >= self.capacity() {
            self.reserve(1)?;
        }

        let index = self.len();
        let value = value.into();

        self.length += 1;
        self[index] = value;

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use ps_alloc::AllocationError;

    use crate::{Buffer, BufferError};

    #[test]
    fn push_digits() -> Result<(), BufferError> {
        let mut buffer = Buffer::new();

        for i in 0..10 {
            buffer.push(b'0' + i)?;
        }

        assert_eq!(buffer.slice(..), b"0123456789");
        assert_eq!(buffer.len(), 10);
        assert!(buffer.capacity() >= 10);

        Ok(())
    }

    #[test]
    fn push_values() -> Result<(), BufferError> {
        let mut buffer = Buffer::new();
        buffer.push(65)?;
        buffer.push(66)?;
        assert_eq!(buffer.slice(..), &[65, 66]);
        assert_eq!(buffer.len(), 2);
        Ok(())
    }

    #[test]
    fn push_at_capacity() -> Result<(), BufferError> {
        // Create a buffer with exact capacity
        let mut buffer = Buffer::with_capacity(2)?;
        buffer.push(1)?;
        buffer.push(2)?;
        // Push one more to trigger resize
        buffer.push(3)?;
        assert_eq!(buffer.slice(..), &[1, 2, 3]);
        assert_eq!(buffer.len(), 3);
        assert!(buffer.capacity() >= 3);
        Ok(())
    }

    #[test]
    fn push_empty() -> Result<(), BufferError> {
        let mut buffer = Buffer::new();
        buffer.push(0)?;
        assert_eq!(buffer.slice(..), &[0]);
        assert_eq!(buffer.len(), 1);
        Ok(())
    }

    #[test]
    fn push_overflow() {
        let mut buffer = Buffer::new();

        buffer.length = usize::MAX;

        let result = buffer.push(0);

        assert!(
            matches!(
                result,
                Err(BufferError::AllocationError(AllocationError::OutOfMemory))
            ),
            "Overflowing the Buffer did not return AllocationError::OutOfMemory."
        );
    }
}

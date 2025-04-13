use crate::Buffer;

impl Buffer {
    /// Removes and returns the last byte from this [`Buffer`], if any.
    ///
    /// This method retrieves the byte at the end of this [`Buffer`] and reduces the buffer's length by one.
    /// If the buffer is empty, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ps_buffer::{Buffer, BufferError};
    /// let mut buffer = Buffer::new();
    /// buffer.push(42)?;
    /// buffer.push(65)?;
    /// assert_eq!(buffer.pop(), Some(65));
    /// assert_eq!(buffer.pop(), Some(42));
    /// assert_eq!(buffer.pop(), None);
    /// # Ok::<(), BufferError>(())
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<u8> {
        if self.is_empty() {
            return None;
        }

        let index = self.len().checked_sub(1)?;
        let value = self[index];

        self.length = self.length.checked_sub(1)?;

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Buffer, BufferError};

    #[test]
    fn pop_values() -> Result<(), BufferError> {
        let mut buffer = Buffer::new();
        buffer.push(42)?;
        buffer.push(65)?;

        assert_eq!(buffer.pop(), Some(65));
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.pop(), Some(42));
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.pop(), None);
        assert_eq!(buffer.len(), 0);

        Ok(())
    }

    #[test]
    fn pop_empty() {
        let mut buffer = Buffer::new();
        assert_eq!(buffer.pop(), None);
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn pop_single() -> Result<(), BufferError> {
        let mut buffer = Buffer::new();
        buffer.push(100)?;
        assert_eq!(buffer.pop(), Some(100));
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.pop(), None);
        Ok(())
    }

    #[test]
    fn pop_multiple() -> Result<(), BufferError> {
        let mut buffer = Buffer::new();
        for i in 0..5 {
            buffer.push(i)?;
        }

        for i in (0..5).rev() {
            assert_eq!(buffer.pop(), Some(i));
        }
        assert_eq!(buffer.pop(), None);
        assert_eq!(buffer.len(), 0);

        Ok(())
    }

    #[test]
    fn pop_underflow() -> Result<(), BufferError> {
        let mut buffer = Buffer::new();

        buffer.push(12)?;
        buffer.length = 0;

        assert_eq!(buffer.pop(), None);

        Ok(())
    }
}

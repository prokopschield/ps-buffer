use ps_range::PartialRange;

use crate::Buffer;

impl Buffer {
    /// Extracts a slice of this buffer, clamping indices to valid bounds.
    pub fn slice(&self, range: impl PartialRange) -> &[u8] {
        let length = self.len();
        let range = range.clamp_right(length);

        &self[range]
    }
}

#[cfg(test)]
mod tests {
    use crate::{Buffer, BufferError};

    #[test]
    fn try_read() -> Result<(), BufferError> {
        let buffer = Buffer::from_slice("Hello, world!")?;

        assert_eq!(buffer.slice(7..=11), b"world");

        Ok(())
    }
}

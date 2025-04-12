use ps_range::PartialRange;

use crate::Buffer;

impl Buffer {
    /// Extracts a mutable slice of this buffer, clamping indices to valid bounds.
    pub fn slice_mut(&mut self, range: impl PartialRange) -> &mut [u8] {
        let length = self.len();
        let range = range.clamp_right(length);

        &mut self[range]
    }
}

#[cfg(test)]
mod tests {
    use crate::{Buffer, BufferError};

    #[test]
    fn try_mutate() -> Result<(), BufferError> {
        let mut buffer = Buffer::alloc(1000)?;

        let slice = buffer.slice_mut(200..);

        slice[13..26].copy_from_slice(b"Hello, world!");

        assert_eq!(&buffer[213..226], b"Hello, world!");

        Ok(())
    }
}

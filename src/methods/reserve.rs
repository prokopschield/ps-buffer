use ps_alloc::HEADER_SIZE;

use crate::{Buffer, BufferError};

impl Buffer {
    /// Reserves at least `self.len() + num_bytes` bytes.
    /// # Errors
    /// - `BufferError` is propagated from [`Buffer::reserve_total`]
    pub fn reserve(&mut self, num_bytes: usize) -> Result<&mut Self, BufferError> {
        let size = self
            .len()
            .saturating_add(num_bytes)
            .saturating_add(HEADER_SIZE)
            .min(MAX_SIZE)
            .next_power_of_two()
            .saturating_sub(HEADER_SIZE);

        self.reserve_total(size)
    }
}

const MAX_SIZE: usize = ((isize::MAX >> 1) as usize).next_power_of_two() - HEADER_SIZE;

#[cfg(test)]
mod tests {
    use ps_alloc::HEADER_SIZE;

    use crate::{Buffer, BufferError};

    #[test]
    fn capacity() -> Result<(), BufferError> {
        let mut buffer = Buffer::new();

        for i in 8..32 {
            buffer.reserve(1 << i)?;

            assert_eq!(
                buffer.capacity(),
                (1 << (i + 1)) - HEADER_SIZE,
                "Incorrect capacity at i={i}"
            );
        }

        Ok(())
    }
}

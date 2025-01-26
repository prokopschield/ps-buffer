use crate::{Buffer, BufferError};

impl Buffer {
    /// Ensures the are at least a total of `capacity` bytes reserved.
    pub fn reserve_total(&mut self, capacity: usize) -> Result<&mut Self, BufferError> {
        if capacity > self.capacity() {
            self.realloc(capacity)?;
        }

        Ok(self)
    }
}

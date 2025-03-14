use crate::{Buffer, BufferError};

impl Buffer {
    /// Ensures the are at least a total of `capacity` bytes reserved.
    /// # Errors
    /// - `AllocationError` is returned if allocation fails.
    /// - `DeallocationError` is returned if deallocation fails.
    pub fn reserve_total(&mut self, capacity: usize) -> Result<&mut Self, BufferError> {
        if capacity > self.capacity() {
            self.realloc(capacity)?;
        }

        Ok(self)
    }
}

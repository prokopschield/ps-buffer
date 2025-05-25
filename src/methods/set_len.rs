use crate::{Buffer, BufferError};

impl Buffer {
    /// Modifies the length of this buffer **without initialization**.
    /// # Errors
    /// - `AllocationError` is returned if allocation fails.
    /// - `DeallocationError` is returned if deallocation fails.
    pub fn set_len(&mut self, new_len: usize) -> Result<&mut Self, BufferError> {
        if new_len > self.capacity() {
            self.realloc(new_len)?;
        }

        self.length = new_len;

        Ok(self)
    }
}

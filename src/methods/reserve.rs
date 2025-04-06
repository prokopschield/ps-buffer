use crate::{Buffer, BufferError};

impl Buffer {
    #[inline]
    /// Reserves `self.length + additional` bytes.
    /// # Errors
    /// `AllocationError` is returned if allocation fails.
    pub fn reserve(&mut self, additional: usize) -> Result<&mut Self, BufferError> {
        self.reserve_total(self.len() + additional)
    }
}

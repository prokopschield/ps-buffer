use crate::{Buffer, BufferError};

impl Buffer {
    /// # Errors
    /// - `AllocationError` is returned if allocation fails.
    /// - `DeallocationError` is returned if deallocation fails.
    pub fn extend_from_slice(&mut self, other: &[u8]) -> Result<&mut Self, BufferError> {
        if other.len() == 0 {
            return Ok(self);
        }

        self.reserve(other.len())?;

        unsafe {
            std::ptr::copy_nonoverlapping(other.as_ptr(), self.ptr.add(self.len()), other.len());
        }

        self.set_len(self.len() + other.len())
    }
}

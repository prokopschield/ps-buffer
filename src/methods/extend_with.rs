use crate::{Buffer, BufferError};

impl Buffer {
    /// # Errors
    /// - `AllocationError` is returned if allocation fails.
    /// - `DeallocationError` is returned if deallocation fails.
    pub fn extend_with(&mut self, n: usize, value: u8) -> Result<&mut Self, BufferError> {
        self.reserve(n)?;

        unsafe {
            std::ptr::write_bytes(self.ptr.add(self.len()), value, n);
        }

        self.set_len(self.len() + n)
    }
}

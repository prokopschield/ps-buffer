use crate::{Buffer, BufferError};

impl Buffer {
    /// # Errors
    /// - `AllocationError` is returned if allocation fails.
    /// - `DeallocationError` is returned if deallocation fails.
    pub fn extend_from_slice<T>(&mut self, other: T) -> Result<&mut Self, BufferError>
    where
        T: AsRef<[u8]>,
    {
        let other = other.as_ref();

        if other.is_empty() {
            return Ok(self);
        }

        self.reserve(other.len())?;

        unsafe {
            std::ptr::copy_nonoverlapping(other.as_ptr(), self.ptr.add(self.len()), other.len());
        }

        self.set_len(self.len() + other.len())
    }
}

use crate::{Buffer, BufferError};

impl Buffer {
    pub fn extend_from_slice(&mut self, other: &[u8]) -> Result<&mut Self, BufferError> {
        self.reserve(other.len())?;

        unsafe {
            std::ptr::copy_nonoverlapping(other.as_ptr(), self.ptr.add(self.len()), other.len());
        }

        self.set_len(self.len() + other.len())
    }
}

use ps_alloc::{alloc, free};

use crate::{Buffer, BufferError};

impl Buffer {
    /// # Errors
    /// - `AllocationError` is returned if allocation fails.
    /// - `DeallocationError` is returned if deallocation fails.
    pub fn realloc(&mut self, size: usize) -> Result<&mut Self, BufferError> {
        let old_ptr = self.ptr;
        let new_ptr = alloc(size)?;

        self.ptr = new_ptr;
        self.capacity = size;
        self.length = self.length.min(self.capacity);

        if !old_ptr.is_null() {
            unsafe {
                std::ptr::copy_nonoverlapping(old_ptr, new_ptr, self.len());
            }

            free(old_ptr)?;
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Buffer, BufferError};

    #[test]
    fn reallocation() -> Result<(), BufferError> {
        let mut buffer = Buffer::default();

        buffer.realloc(128)?;
        buffer.set_len(128)?;

        for i in buffer.as_mut() {
            *i = std::ptr::addr_of!(i) as u8;
        }

        let copy = Buffer::from(&buffer)?;

        buffer.realloc(512)?;

        assert_eq!(buffer[0..128], *copy);

        buffer.realloc(64)?;

        assert_eq!(buffer[0..64], copy[0..64]);

        Ok(())
    }
}

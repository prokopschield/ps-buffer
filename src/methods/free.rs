use std::ptr::null_mut;

use ps_alloc::{free, DeallocationError};

use crate::Buffer;

impl Buffer {
    /// Deallocates this `Buffer`.
    pub fn free(&mut self) -> Result<&mut Self, DeallocationError> {
        let ptr = self.ptr;

        self.capacity = 0;
        self.length = 0;
        self.ptr = null_mut();

        free(ptr)?;

        Ok(self)
    }
}

use crate::{Buffer, BufferError};

impl Buffer {
    #[inline]
    pub fn reserve(&mut self, additional: usize) -> Result<&mut Self, BufferError> {
        self.reserve_total(self.len() + additional)
    }
}

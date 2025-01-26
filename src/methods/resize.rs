use crate::{Buffer, BufferError};

impl Buffer {
    pub fn resize(&mut self, new_len: usize, value: u8) -> Result<&mut Self, BufferError> {
        let len = self.len();

        if new_len <= len {
            Ok(self.truncate(new_len))
        } else {
            self.extend_with(new_len - len, value)
        }
    }
}

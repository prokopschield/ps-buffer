use crate::{Buffer, SharedBuffer};

impl Drop for SharedBuffer {
    fn drop(&mut self) {
        let mut buffer = Buffer::new();
        std::mem::swap(&mut self.buffer, &mut buffer);
        std::mem::forget(buffer);
    }
}

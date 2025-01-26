use std::fmt::Debug;

use crate::Buffer;

impl Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("length", &self.len())
            .field("capacity", &self.capacity())
            .field("bytes", &&self[..])
            .finish()
    }
}

use std::fmt::Debug;

use crate::SharedBuffer;

impl Debug for SharedBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.buffer.fmt(f)
    }
}

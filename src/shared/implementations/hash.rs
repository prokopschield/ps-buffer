use std::hash::Hash;

use crate::SharedBuffer;

impl Hash for SharedBuffer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buffer.hash(state);
    }
}

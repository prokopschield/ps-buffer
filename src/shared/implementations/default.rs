use crate::{Buffer, SharedBuffer};

impl Default for SharedBuffer {
    fn default() -> Self {
        Self::from(Buffer::default())
    }
}

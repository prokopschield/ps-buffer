use std::ops::Deref;

use crate::{Buffer, SharedBuffer};

impl Deref for SharedBuffer {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

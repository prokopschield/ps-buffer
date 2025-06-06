use std::ops::Deref;

use crate::Buffer;

impl Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        if self.is_empty() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len()) }
        }
    }
}

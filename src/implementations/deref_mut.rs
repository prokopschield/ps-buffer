use std::ops::DerefMut;

use crate::Buffer;

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.is_empty() {
            &mut []
        } else {
            unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
        }
    }
}

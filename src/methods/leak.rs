use std::slice::from_raw_parts_mut;

use crate::Buffer;

impl Buffer {
    /// Leaks this Buffer's memory. Caller shall ensure the leaked memory is later freed with correct alignment.
    #[must_use]
    pub fn leak(self) -> &'static mut [u8] {
        let len = self.len();
        let ptr = self.ptr;

        std::mem::forget(self);

        unsafe { from_raw_parts_mut(ptr, len) }
    }
}

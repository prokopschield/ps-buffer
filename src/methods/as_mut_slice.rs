#![allow(clippy::missing_const_for_fn)]

use crate::Buffer;

impl Buffer {
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self
    }
}

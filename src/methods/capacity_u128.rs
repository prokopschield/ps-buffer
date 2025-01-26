use crate::{constants::FACTOR, Buffer};

impl Buffer {
    #[inline]
    pub fn capacity_u128(&self) -> usize {
        self.capacity().div_ceil(FACTOR)
    }
}

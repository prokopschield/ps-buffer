use crate::{constants::FACTOR, Buffer};

impl Buffer {
    #[inline]
    #[must_use]
    pub const fn capacity_u128(&self) -> usize {
        self.capacity().div_ceil(FACTOR)
    }
}

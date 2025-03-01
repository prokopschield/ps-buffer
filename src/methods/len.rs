use crate::Buffer;

impl Buffer {
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.length
    }
}

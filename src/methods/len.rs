use crate::Buffer;

impl Buffer {
    #[inline]
    pub const fn len(&self) -> usize {
        self.length
    }
}

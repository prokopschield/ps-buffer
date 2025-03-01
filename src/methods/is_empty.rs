use crate::Buffer;

impl Buffer {
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

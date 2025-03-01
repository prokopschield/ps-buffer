use crate::Buffer;

impl Buffer {
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self
    }
}

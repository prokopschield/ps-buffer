use crate::Buffer;

impl Buffer {
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self
    }
}

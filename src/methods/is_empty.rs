use crate::Buffer;

impl Buffer {
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

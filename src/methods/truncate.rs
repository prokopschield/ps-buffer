use crate::Buffer;

impl Buffer {
    #[inline]
    pub fn truncate(&mut self, len: usize) -> &mut Self {
        if len <= self.length {
            self.length = len
        }

        self
    }
}

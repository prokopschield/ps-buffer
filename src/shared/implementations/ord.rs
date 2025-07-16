use crate::SharedBuffer;

impl Ord for SharedBuffer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.buffer.cmp(other)
    }
}

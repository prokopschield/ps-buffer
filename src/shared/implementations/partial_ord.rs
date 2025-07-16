use crate::SharedBuffer;

impl PartialOrd for SharedBuffer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

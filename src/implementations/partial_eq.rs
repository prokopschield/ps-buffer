use crate::Buffer;

impl PartialEq for Buffer {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self[..] == other[..]
    }
}

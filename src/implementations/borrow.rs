use std::borrow::Borrow;

use crate::Buffer;

impl Borrow<[u8]> for Buffer {
    fn borrow(&self) -> &[u8] {
        self
    }
}

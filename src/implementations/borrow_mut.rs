use std::borrow::BorrowMut;

use crate::Buffer;

impl BorrowMut<[u8]> for Buffer {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self
    }
}

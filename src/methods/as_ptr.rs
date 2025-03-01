use crate::Buffer;

impl Buffer {
    #[inline]
    #[must_use]
    pub const fn as_ptr(&self) -> *const u8 {
        self.ptr
    }
}

#[cfg(test)]
mod tests {
    use crate::Buffer;

    #[test]
    fn identity() {
        const SIZE: usize = 300;

        let mut vec = Vec::<u8>::with_capacity(SIZE);

        let buffer = Buffer {
            capacity: vec.capacity(),
            length: vec.len(),
            ptr: vec.as_mut_ptr(),
        };

        assert_eq!(buffer.as_ptr(), vec.as_ptr().cast::<u8>());

        std::mem::forget(buffer);
    }
}

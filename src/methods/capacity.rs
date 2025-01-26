use crate::Buffer;

impl Buffer {
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use crate::Buffer;

    #[test]
    fn are_equal() {
        let buffer = Buffer::alloc(12).unwrap();

        assert_eq!(buffer.capacity, buffer.capacity());
    }
}

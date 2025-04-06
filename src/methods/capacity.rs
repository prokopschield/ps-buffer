use crate::Buffer;

impl Buffer {
    #[inline]
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::Buffer;

    #[test]
    fn are_equal() {
        let buffer = Buffer::alloc(12).unwrap();

        assert_eq!(buffer.capacity, buffer.capacity());
    }
}

use crate::Buffer;

impl Default for Buffer {
    #[inline]
    /// Creates an empty Buffer.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Buffer;

    #[test]
    fn is_null() {
        let buffer = Buffer::default();

        assert_eq!(buffer.ptr, std::ptr::null_mut());
    }
}

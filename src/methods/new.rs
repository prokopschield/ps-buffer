use crate::Buffer;

impl Buffer {
    #[inline]
    /// Creates an empty Buffer.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            capacity: 0,
            length: 0,
            ptr: std::ptr::null_mut(),
        }
    }
}

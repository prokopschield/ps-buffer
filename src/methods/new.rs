use crate::Buffer;

impl Buffer {
    #[inline]
    /// Creates an empty Buffer.
    pub fn new() -> Self {
        Self {
            capacity: 0,
            length: 0,
            ptr: std::ptr::null_mut(),
        }
    }
}

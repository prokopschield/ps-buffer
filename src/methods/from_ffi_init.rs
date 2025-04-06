use crate::{Buffer, BufferError};

impl Buffer {
    /// Creates a new `Buffer` with custom initialization.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The number of bytes to allocate.
    /// * `init_fn` - A closure that initializes the buffer and returns the number of bytes actually used.
    ///
    /// # Returns
    ///
    /// `Ok(Buffer)` with at least `bytes` allocated, where `init_fn` has been applied to initialize some portion of it,
    /// or `Err(BufferError)` if allocation fails.
    ///
    /// # Errors
    ///
    /// - `AllocationError` is returned if allocation fails.
    ///
    /// # Panics
    ///
    /// - Panics if the number of initialized bytes > number of allocated bytes.
    pub fn from_ffi_init<F>(bytes: usize, init_fn: F) -> Result<Self, BufferError>
    where
        F: FnOnce(&mut [u8]) -> usize,
    {
        // Allocate the buffer with the specified number of bytes
        let mut buffer = Self::alloc_uninit(bytes)?;

        // Invoke the initialization function
        let initialized_bytes = init_fn(buffer.as_mut());

        // Ensure that the number of bytes initialized does not exceed the allocated size
        debug_assert!(
            initialized_bytes <= bytes,
            "Initialized bytes ({initialized_bytes}) cannot exceed allocated bytes ({bytes})."
        );

        // Truncate uninitialized bytes
        buffer.truncate(initialized_bytes);

        // Return resulting `Buffer`
        Ok(buffer)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ffi_init_full_buffer() {
        let buffer = Buffer::from_ffi_init(10, |slice| {
            for (i, byte) in slice.iter_mut().enumerate() {
                *byte = u8::try_from(i).unwrap();
            }
            slice.len()
        })
        .unwrap();

        // Check if the buffer is fully initialized
        assert_eq!(buffer.len(), 10);
        assert_eq!(&buffer[..], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_from_ffi_init_partial_buffer() {
        let buffer = Buffer::from_ffi_init(10, |slice| {
            for (i, byte) in slice.iter_mut().enumerate() {
                if i < 5 {
                    *byte = u8::try_from(i).unwrap();
                }
            }
            5 // Only 5 bytes are initialized
        })
        .unwrap();

        // Check if the buffer is partially initialized
        assert_eq!(buffer.len(), 5);
        assert_eq!(&buffer[..], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_from_ffi_init_zero_bytes() {
        let buffer = Buffer::from_ffi_init(10, |_slice| {
            0 // No bytes are initialized
        })
        .unwrap();

        // Check if the buffer is empty
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), 10); // Capacity should remain 10
    }

    #[test]
    fn test_from_ffi_init_allocation_failure() {
        // Assuming Buffer::alloc_uninit might fail if asking for a very large buffer
        let result = Buffer::from_ffi_init(usize::MAX, |_slice| 0);
        assert!(result.is_err());
    }

    #[test]
    #[should_panic(expected = "Initialized bytes (11) cannot exceed allocated bytes (10).")]
    fn test_from_ffi_init_panic_on_over_initialization() {
        // This test should panic due to the debug_assert
        let _ = Buffer::from_ffi_init(10, |slice| {
            for (i, byte) in slice.iter_mut().enumerate() {
                *byte = u8::try_from(i).unwrap();
            }
            11 // Trying to initialize more bytes than allocated
        })
        .unwrap();
    }
}

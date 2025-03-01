use crate::Buffer;

impl Buffer {
    /// Creates a `Buffer` directly from a pointer, a length, and a capacity.
    ///
    /// # Safety
    ///
    /// This is highly unsafe due to the number of invariants that aren't
    /// checked:
    ///
    /// * `ptr` must have been allocated using the [`ps_alloc::alloc`] function.
    /// * `length` needs to be less than or equal to `capacity`.
    /// * The first `length` values must be properly initialized.
    /// * `capacity` needs to be the capacity that the pointer was allocated with.
    /// * The allocated size in bytes must be no larger than `isize::MAX`.
    ///   See the safety documentation of [`pointer::offset`].
    ///
    /// These requirements are always upheld by any `ptr` that has been allocated
    /// by `Buffer`. Other allocation sources are allowed if the invariants are
    /// upheld.
    ///
    /// Violating these may cause problems like corrupting the allocator's
    /// internal data structures. `Buffer`'s deallocator will also read bytes
    /// right before the pointer, which may cause a segmentation fault.
    ///
    /// It's also not safe to build one from a non-aligned pointer, because
    /// the allocator cares about the alignment, and `Buffer` uses 16-byte
    /// alignment. `Buffer` always deallocates with alignment 16.
    ///
    /// If you need to create a `Buffer` from memory which does not uphold there invariants,
    /// use [`slice::from_raw_parts`] and [`Buffer::from`].
    ///
    /// The ownership of `ptr` is effectively transferred to the
    /// `Buffer` which may then deallocate, reallocate or change the
    /// contents of memory pointed to by the pointer at will. Ensure
    /// that nothing else uses the pointer after calling this
    /// function.
    ///
    /// *This documenting comment is based on [`Vec::from_raw_parts`].*
    #[inline]
    pub const unsafe fn from_raw_parts(ptr: *mut u8, length: usize, capacity: usize) -> Self {
        Self {
            ptr,
            capacity,
            length,
        }
    }
}

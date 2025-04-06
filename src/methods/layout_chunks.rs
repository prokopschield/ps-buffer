use std::alloc::{Layout, LayoutError};

use crate::{AlignmentType, Buffer};

impl Buffer {
    #[inline]
    /// # Errors
    /// [`LayoutError`] is propagated from [`Layout::array`].
    pub fn layout_chunks(chunks: usize) -> Result<Layout, LayoutError> {
        Layout::array::<AlignmentType>(chunks)
    }
}

#[cfg(test)]
mod tests {
    use std::alloc::LayoutError;

    use crate::{constants::FACTOR, Buffer};

    #[test]
    fn layout_size() -> Result<(), LayoutError> {
        const SIZE: usize = 300;

        let layout = Buffer::layout_chunks(SIZE)?;
        let align = layout.align();
        let size = layout.size();

        assert_eq!(align, FACTOR);
        assert_eq!(size, SIZE * FACTOR);

        Ok(())
    }
}

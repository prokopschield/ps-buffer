use std::alloc::{Layout, LayoutError};

use crate::{constants::FACTOR, Buffer};

impl Buffer {
    #[inline]
    /// # Errors
    /// [`LayoutError`] is propagated from [`Layout::array`].
    pub fn layout_bytes(bytes: usize) -> Result<Layout, LayoutError> {
        Self::layout_chunks(bytes.div_ceil(FACTOR))
    }
}

#[cfg(test)]
mod tests {
    use std::alloc::LayoutError;

    use crate::{constants::FACTOR, Buffer};

    #[test]
    fn layout_size() -> Result<(), LayoutError> {
        const SIZE: usize = 512;

        let layout = Buffer::layout_bytes(SIZE)?;
        let align = layout.align();
        let size = layout.size();

        assert_eq!(align, FACTOR);
        assert_eq!(size, SIZE);

        Ok(())
    }
}

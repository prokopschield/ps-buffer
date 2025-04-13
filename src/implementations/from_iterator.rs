use crate::Buffer;

impl FromIterator<u8> for Buffer {
    /// Creates a [`Buffer`] from an iterator of `u8` values.
    ///
    /// This method collects bytes from the iterator into a new [`Buffer`].
    /// It attempts to pre-allocate capacity based on the iterator's size hint
    /// to minimize reallocations. If any push operation fails (e.g., due to
    /// allocation failure or length overflow), the buffer will contain all
    /// bytes successfully pushed up to that point, and further elements are
    /// ignored.
    ///
    /// # Arguments
    ///
    /// * `iter` - An iterator yielding `u8` values
    ///
    /// # Returns
    ///
    /// A [`Buffer`] containing as many bytes as could be successfully pushed.
    /// If no errors occur, the buffer contains all bytes from the iterator.
    /// If an error occurs, the buffer may be partial.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ps_buffer::{Buffer, BufferError};
    /// let bytes = vec![42, 65, 66];
    /// let buffer: Buffer = bytes.into_iter().collect();
    /// assert_eq!(buffer.slice(..), &[42, 65, 66]);
    ///
    /// let empty: Buffer = b"".into_iter().collect();
    /// assert!(empty.is_empty());
    /// # Ok::<(), BufferError>(())
    /// ```
    ///
    /// # Notes
    ///
    /// If you need to detect whether all elements were collected successfully,
    /// consider using a different method that can propagate errors, as this
    /// implementation returns a partial buffer on failure.
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let iterator = iter.into_iter();
        let (lower, _upper) = iterator.size_hint();

        // Start with at least the lower bound capacity
        let mut buffer = Self::with_capacity(lower).unwrap_or_default();

        for byte in iterator {
            if buffer.push(byte).is_err() {
                break;
            }
        }

        buffer
    }
}

impl<'lt> FromIterator<&'lt u8> for Buffer {
    fn from_iter<T: IntoIterator<Item = &'lt u8>>(iter: T) -> Self {
        iter.into_iter().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Buffer;

    #[test]
    fn collect_empty_iter() {
        let buffer: Buffer = b"".iter().collect();
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn collect_vec_iter() {
        let bytes = vec![42, 65, 66];
        let buffer: Buffer = bytes.into_iter().collect();
        assert_eq!(buffer.slice(..), &[42, 65, 66]);
        assert_eq!(buffer.len(), 3);
    }

    #[test]
    fn collect_range_iter() {
        let buffer: Buffer = (0..5).collect();
        assert_eq!(buffer.slice(..), &[0, 1, 2, 3, 4]);
        assert_eq!(buffer.len(), 5);
    }

    #[test]
    fn collect_capacity_allocation() {
        let buffer: Buffer = (0..100).collect();
        assert_eq!(buffer.len(), 100);
        assert!(buffer.capacity() >= 100);
    }

    #[test]
    fn collect_from_array() {
        let buffer: Buffer = [1, 2, 3].into_iter().collect();
        assert_eq!(buffer.slice(..), &[1, 2, 3]);
        assert_eq!(buffer.len(), 3);
    }

    #[test]
    fn collect_from_vec_empty() {
        let bytes: Vec<u8> = vec![];
        let buffer: Buffer = bytes.into_iter().collect();
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
    }
}

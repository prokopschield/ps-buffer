use crate::{Buffer, BufferError, SharedBuffer};

pub trait ByteIteratorIntoSharedBuffer {
    /// Collects this [`Iterator`] into a [`SharedBuffer`].
    /// # Errors
    /// [`BufferError`] is returned if an allocation or reallocation fails.
    fn into_shared_buffer(self) -> Result<SharedBuffer, BufferError>;
}

impl<T: Iterator<Item = u8>> ByteIteratorIntoSharedBuffer for T {
    fn into_shared_buffer(self) -> Result<SharedBuffer, BufferError> {
        let mut buffer = Buffer::with_capacity(self.size_hint().0)?;

        for byte in self {
            buffer.push(byte)?;
        }

        Ok(buffer.share())
    }
}

pub trait ByteRefIteratorIntoSharedBuffer {
    /// Collects this [`Iterator`] into a [`SharedBuffer`].
    /// # Errors
    /// [`BufferError`] is returned if an allocation or reallocation fails.
    fn into_shared_buffer(self) -> Result<SharedBuffer, BufferError>;
}

impl<'lt, T: Iterator<Item = &'lt u8>> ByteRefIteratorIntoSharedBuffer for T {
    fn into_shared_buffer(self) -> Result<SharedBuffer, BufferError> {
        self.copied().into_shared_buffer()
    }
}

pub trait ByteSliceIteratorIntoSharedBuffer {
    /// Collects this [`Iterator`] into a [`SharedBuffer`].
    /// # Errors
    /// [`BufferError`] is returned if an allocation or reallocation fails.
    fn into_shared_buffer(self) -> Result<SharedBuffer, BufferError>;
}

impl<Ts: AsRef<[u8]>, Ti: Iterator<Item = Ts>> ByteSliceIteratorIntoSharedBuffer for Ti {
    fn into_shared_buffer(self) -> Result<SharedBuffer, BufferError> {
        let mut buffer = Buffer::new();

        for bytes in self {
            buffer.extend_from_slice(bytes.as_ref())?;
        }

        Ok(buffer.share())
    }
}

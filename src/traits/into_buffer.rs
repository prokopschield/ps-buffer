use crate::{Buffer, BufferError};

pub trait ByteIteratorIntoBuffer {
    /// Collects this [`Iterator`] into a [`Buffer`].
    /// # Errors
    /// [`BufferError`] is returned if an allocation or reallocation fails.
    fn into_buffer(self) -> Result<Buffer, BufferError>;
}

impl<T: Iterator<Item = u8>> ByteIteratorIntoBuffer for T {
    fn into_buffer(self) -> Result<Buffer, BufferError> {
        let mut buffer = Buffer::with_capacity(self.size_hint().0)?;

        for byte in self {
            buffer.push(byte)?;
        }

        Ok(buffer)
    }
}

pub trait ByteRefIteratorIntoBuffer {
    /// Collects this [`Iterator`] into a [`Buffer`].
    /// # Errors
    /// [`BufferError`] is returned if an allocation or reallocation fails.
    fn into_buffer(self) -> Result<Buffer, BufferError>;
}

impl<'lt, T: Iterator<Item = &'lt u8>> ByteRefIteratorIntoBuffer for T {
    fn into_buffer(self) -> Result<Buffer, BufferError> {
        self.copied().into_buffer()
    }
}

pub trait ByteSliceIteratorIntoBuffer {
    /// Collects this [`Iterator`] into a [`Buffer`].
    /// # Errors
    /// [`BufferError`] is returned if an allocation or reallocation fails.
    fn into_buffer(self) -> Result<Buffer, BufferError>;
}

impl<Ts: AsRef<[u8]>, Ti: Iterator<Item = Ts>> ByteSliceIteratorIntoBuffer for Ti {
    fn into_buffer(self) -> Result<Buffer, BufferError> {
        let mut buffer = Buffer::new();

        for bytes in self {
            buffer.extend_from_slice(bytes.as_ref())?;
        }

        Ok(buffer)
    }
}

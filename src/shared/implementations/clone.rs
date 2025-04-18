use crate::{Buffer, SharedBuffer};

impl Clone for SharedBuffer {
    fn clone(&self) -> Self {
        Self {
            arc: self.arc.clone(),
            buffer: Buffer {
                capacity: self.buffer.capacity,
                length: self.buffer.length,
                ptr: self.buffer.ptr,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use ps_alloc::DeallocationError;

    use crate::{BufferError, SharedBuffer, ToBuffer};

    #[test]
    fn is_freed() -> Result<(), BufferError> {
        let buffer1: SharedBuffer = "Hello, world!".to_buffer()?.into();
        let buffer2 = buffer1.clone();
        let buffer3 = buffer2.clone();

        assert_eq!(&buffer1.slice(..), b"Hello, world!");
        assert_eq!(&buffer2.slice(..), b"Hello, world!");
        assert_eq!(&buffer3.slice(..), b"Hello, world!");

        drop(buffer3);
        drop(buffer1);

        assert_eq!(&buffer2.slice(..), b"Hello, world!");

        let ptr = buffer2.ptr;

        drop(buffer2);

        assert!(matches!(
            ps_alloc::free(ptr),
            Err(DeallocationError::DoubleFree | DeallocationError::InvalidAllocation)
        ));

        Ok(())
    }
}

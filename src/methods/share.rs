use crate::{Buffer, SharedBuffer};

impl Buffer {
    /// Constructs a [`SharedBuffer`] containing this [`Buffer`].
    #[must_use]
    pub fn share(self) -> SharedBuffer {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{BufferError, ToBuffer};

    #[test]
    fn hello_shared() -> Result<(), BufferError> {
        let str = "Hello, shared buffer!";
        let shared = str.to_buffer()?.share();
        let clone = shared.clone();

        assert_eq!(shared.slice(..), clone.slice(..));

        drop(shared);

        assert_eq!(clone.slice(..), str.as_bytes());

        Ok(())
    }
}

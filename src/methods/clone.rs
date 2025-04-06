use crate::{Buffer, BufferError};

impl Buffer {
    #[allow(clippy::should_implement_trait)]
    pub fn clone(&self) -> Result<Self, BufferError> {
        Self::from(&self[..])
    }
}

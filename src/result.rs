use crate::{Buffer, BufferError};

#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub enum BufferResult {
    Ok(Buffer),
    Err(BufferError),
}

impl Clone for BufferResult {
    fn clone(&self) -> Self {
        match self {
            Self::Ok(buffer) => buffer.clone().into(),
            Self::Err(err) => Self::Err(err.clone()),
        }
    }
}

impl Default for BufferResult {
    fn default() -> Self {
        Self::Ok(Buffer::default())
    }
}

impl From<Result<Buffer, BufferError>> for BufferResult {
    fn from(result: Result<Buffer, BufferError>) -> Self {
        match result {
            Ok(buffer) => Self::Ok(buffer),
            Err(err) => Self::Err(err),
        }
    }
}

impl From<Buffer> for BufferResult {
    fn from(buffer: Buffer) -> Self {
        Self::Ok(buffer)
    }
}

impl From<BufferError> for BufferResult {
    fn from(err: BufferError) -> Self {
        Self::Err(err)
    }
}

impl From<BufferResult> for Result<Buffer, BufferError> {
    fn from(result: BufferResult) -> Self {
        match result {
            BufferResult::Ok(buffer) => Ok(buffer),
            BufferResult::Err(err) => Err(err),
        }
    }
}

impl BufferResult {
    #[allow(clippy::missing_errors_doc)]
    /// Converts this [`BufferResult`] into a [`std::result::Result`].
    pub fn into_result(self) -> Result<Buffer, BufferError> {
        self.into()
    }
}

use crate::{Buffer, BufferError};

#[derive(Debug, PartialEq, Eq)]
pub enum Result {
    Ok(Buffer),
    Err(BufferError),
}

impl Clone for Result {
    fn clone(&self) -> Self {
        match self {
            Self::Ok(buffer) => buffer.clone().into(),
            Self::Err(err) => Self::Err(err.clone()),
        }
    }
}

impl Default for Result {
    fn default() -> Self {
        Self::Ok(Buffer::default())
    }
}

impl From<std::result::Result<Buffer, BufferError>> for Result {
    fn from(result: std::result::Result<Buffer, BufferError>) -> Self {
        match result {
            Ok(buffer) => Self::Ok(buffer),
            Err(err) => Self::Err(err),
        }
    }
}

impl From<Buffer> for Result {
    fn from(buffer: Buffer) -> Self {
        Self::Ok(buffer)
    }
}

impl From<BufferError> for Result {
    fn from(err: BufferError) -> Self {
        Self::Err(err)
    }
}

impl From<Result> for std::result::Result<Buffer, BufferError> {
    fn from(result: Result) -> Self {
        match result {
            Result::Ok(buffer) => Ok(buffer),
            Result::Err(err) => Err(err),
        }
    }
}

impl Result {
    #[allow(clippy::missing_errors_doc)]
    /// Converts this [`crate::BufferOps::Result`] into a [`std::result::Result`].
    pub fn into_result(self) -> std::result::Result<Buffer, BufferError> {
        self.into()
    }
}

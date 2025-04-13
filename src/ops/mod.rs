use crate::{Buffer, BufferError};

pub trait BufferOps {
    type Error;
    type Result;
}

impl BufferOps for Buffer {
    type Error = BufferError;
    type Result = Result<Self, Self::Error>;
}

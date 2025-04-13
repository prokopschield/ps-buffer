use crate::{Buffer, BufferError, Result};

pub trait BufferOps {
    type Error;
    type Result;
}

impl BufferOps for Buffer {
    type Error = BufferError;
    type Result = Result;
}

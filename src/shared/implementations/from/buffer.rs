#![allow(clippy::unnecessary_struct_initialization)]

use std::sync::Arc;

use crate::{Buffer, SharedBuffer};

impl From<Buffer> for SharedBuffer {
    fn from(value: Buffer) -> Self {
        let buffer = Buffer {
            capacity: value.capacity,
            length: value.length,
            ptr: value.ptr,
        };

        Self {
            arc: Arc::new(value),
            buffer,
        }
    }
}

#![allow(clippy::module_name_repetitions)]

pub use constants::*;
pub use error::BufferError;
pub use shared::SharedBuffer;
pub use traits::*;

mod constants;
mod error;
mod implementations;
mod methods;
mod shared;
mod traits;

#[must_use]
pub struct Buffer {
    /// This is a raw pointer to this Buffer's data.
    ptr: *mut u8,
    /// Buffer's allocated capacity in bytes.
    capacity: usize,
    /// Buffer length in bytes.
    length: usize,
}

unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}

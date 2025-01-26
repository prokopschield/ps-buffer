pub use constants::*;
pub use error::BufferError;

mod constants;
mod error;
mod implementations;
mod methods;

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

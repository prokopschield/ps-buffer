use std::sync::Arc;

use crate::Buffer;

mod implementations;

/// [`SharedBuffer`] is a thread-safe shared-ownership wrapper around a [`Buffer`].
///
/// The underlying [`Buffer`] is only dropped after the underlying [`Arc`] is dropped.
pub struct SharedBuffer {
    arc: Arc<Buffer>,
    buffer: Buffer,
}

use std::hash::Hash;

use crate::Buffer;

impl Hash for Buffer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self[..].hash(state);
    }
}

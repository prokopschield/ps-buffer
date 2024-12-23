use std::ops::{Deref, DerefMut};

pub type AlignmentType = u128;
pub const FILLER: AlignmentType = 0;

#[derive(Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Buffer {
    vec: Vec<AlignmentType>,
    length: usize,
}

impl Buffer {
    pub fn alloc(length: usize) -> Self {
        let vec_len = helpers::to_vec_len(length);
        let vec: Vec<AlignmentType> = vec![FILLER; vec_len];

        Self { vec, length }
    }

    pub fn from<T: AsRef<[u8]>>(value: T) -> Self {
        let source = value.as_ref();

        let mut buffer = Self::alloc(source.len());

        buffer.copy_from_slice(source);

        buffer
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn resize(&mut self, length: usize) {
        if length < self.length {
            self.length = length;
            return;
        }

        let vec_len = helpers::to_vec_len(length);

        if vec_len <= self.vec.len() {
            let old_len = self.length;
            self.length = length;
            self[old_len..length].fill(0);
        } else {
            let mut new_buf = Self::alloc(length);
            new_buf[0..self.len()].copy_from_slice(self);
            self.vec = new_buf.vec;
            self.length = new_buf.length;
        }
    }
}

impl AsMut<[u8]> for Buffer {
    fn as_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.vec.as_ptr() as *const u8, self.len()) }
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.vec.as_ptr() as *mut u8, self.len()) }
    }
}

impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("length", &self.len())
            .field("bytes", &&self[..])
            .finish()
    }
}

pub mod helpers {
    pub fn to_vec_len(length: usize) -> usize {
        (length - 1) / std::mem::size_of::<crate::AlignmentType>() + 1
    }
}

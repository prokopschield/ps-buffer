use std::ops::{Deref, DerefMut};

pub type AlignmentType = u128;
pub const FILLER: AlignmentType = 0;

pub struct Buffer {
    vec: Vec<AlignmentType>,
    length: usize,
}

impl Buffer {
    pub fn alloc(length: usize) -> Self {
        let vec_len = (length - 1) / std::mem::size_of::<AlignmentType>() + 1;
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

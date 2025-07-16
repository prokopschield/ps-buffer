use crate::SharedBuffer;

impl PartialEq for SharedBuffer {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

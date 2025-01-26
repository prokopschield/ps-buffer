use crate::Buffer;

impl Drop for Buffer {
    fn drop(&mut self) {
        let _ = self.free();
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::Buffer;

    #[test]
    fn dealloc() -> Result<(), Box<dyn Error>> {
        let mut buffer = Buffer::default();

        buffer.resize(120, 7)?;

        Ok(())
    }
}

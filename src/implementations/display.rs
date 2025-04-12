use std::fmt::{Display, Formatter, Result};

use crate::Buffer;

impl Display for Buffer {
    /// Prints this buffer in Node.js notation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<Buffer ")?;

        for (i, byte) in self.as_slice().iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{byte:02x}")?;
        }

        write!(f, ">")
    }
}

#[cfg(test)]
mod tests {
    use crate::{Buffer, BufferError};

    #[test]
    pub fn deadbeef() -> Result<(), BufferError> {
        let buffer = Buffer::from_slice(b"\xde\xad\xbe\xef")?;
        let pretty = buffer.to_string();

        assert_eq!(pretty, "<Buffer de ad be ef>");

        Ok(())
    }
}

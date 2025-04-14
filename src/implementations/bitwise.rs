use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};

// Macro for element-wise bitwise operations with &[u8]
macro_rules! impl_bitwise_op_slice {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<&[u8]> for crate::Buffer {
            fn $method(&mut self, rhs: &[u8]) {
                let len = self.len().min(rhs.len());
                for i in 0..len {
                    (*self)[i] $op rhs[i];
                }
            }
        }
    };
}

// Macro for bitwise operations with u8
macro_rules! impl_bitwise_op_u8 {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<u8> for crate::Buffer {
            fn $method(&mut self, rhs: u8) {
                for byte in self.iter_mut() {
                    *byte $op rhs;
                }
            }
        }
    };
}

// Macro for bitwise operations with &u8
macro_rules! impl_bitwise_op_ref_u8 {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<&u8> for crate::Buffer {
            fn $method(&mut self, rhs: &u8) {
                for byte in self.iter_mut() {
                    *byte $op *rhs;
                }
            }
        }
    };
}

// Bitwise operations
impl_bitwise_op_slice!(BitAndAssign, bitand_assign, &=);
impl_bitwise_op_slice!(BitOrAssign, bitor_assign, |=);
impl_bitwise_op_slice!(BitXorAssign, bitxor_assign, ^=);

impl_bitwise_op_u8!(BitAndAssign, bitand_assign, &=);
impl_bitwise_op_u8!(BitOrAssign, bitor_assign, |=);
impl_bitwise_op_u8!(BitXorAssign, bitxor_assign, ^=);

impl_bitwise_op_ref_u8!(BitAndAssign, bitand_assign, &=);
impl_bitwise_op_ref_u8!(BitOrAssign, bitor_assign, |=);
impl_bitwise_op_ref_u8!(BitXorAssign, bitxor_assign, ^=);

// Shift operations for u8 only
impl ShlAssign<u8> for crate::Buffer {
    fn shl_assign(&mut self, rhs: u8) {
        for byte in self.iter_mut() {
            *byte <<= rhs;
        }
    }
}

impl ShrAssign<u8> for crate::Buffer {
    fn shr_assign(&mut self, rhs: u8) {
        for byte in self.iter_mut() {
            *byte >>= rhs;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bitand_assign_slice() -> Result<(), crate::BufferError> {
        let mut buf = crate::Buffer::from_slice(&[0b1010, 0b1100])?;
        let rhs = [0b1100, 0b1010];
        buf &= &rhs[..];
        assert_eq!(&*buf, &[0b1000, 0b1000]);
        Ok(())
    }

    #[test]
    fn test_bitand_assign_u8() -> Result<(), crate::BufferError> {
        let mut buf = crate::Buffer::from_slice(&[0b1010, 0b1100])?;
        buf &= 0b1100u8;
        assert_eq!(&*buf, &[0b1000, 0b1100]);
        Ok(())
    }

    #[test]
    fn test_bitand_assign_ref_u8() -> Result<(), crate::BufferError> {
        let mut buf = crate::Buffer::from_slice(&[0b1010, 0b1100])?;
        buf &= &0b1100u8;
        assert_eq!(&*buf, &[0b1000, 0b1100]);
        Ok(())
    }

    #[test]
    fn test_bitor_assign_slice() -> Result<(), crate::BufferError> {
        let mut buf = crate::Buffer::from_slice(&[0b1010, 0b1100])?;
        let rhs = [0b1100, 0b1010];
        buf |= &rhs[..];
        assert_eq!(&*buf, &[0b1110, 0b1110]);
        Ok(())
    }

    #[test]
    fn test_bitor_assign_u8() -> Result<(), crate::BufferError> {
        let mut buf = crate::Buffer::from_slice(&[0b1010, 0b1100])?;
        buf |= 0b1010u8;
        assert_eq!(&*buf, &[0b1010, 0b1110]);
        Ok(())
    }

    #[test]
    fn test_bitor_assign_ref_u8() -> Result<(), crate::BufferError> {
        let mut buf = crate::Buffer::from_slice(&[0b1010, 0b1100])?;
        buf |= &0b1010u8;
        assert_eq!(&*buf, &[0b1010, 0b1110]);
        Ok(())
    }

    #[test]
    fn test_bitxor_assign_slice() -> Result<(), crate::BufferError> {
        let mut buf = crate::Buffer::from_slice(&[0b1010, 0b1100])?;
        let rhs = [0b1100, 0b1010];
        buf ^= &rhs[..];
        assert_eq!(&*buf, &[0b0110, 0b0110]);
        Ok(())
    }

    #[test]
    fn test_bitxor_assign_u8() -> Result<(), crate::BufferError> {
        let mut buf = crate::Buffer::from_slice(&[0b1010, 0b1100])?;
        buf ^= 0b1111u8;
        assert_eq!(&*buf, &[0b0101, 0b0011]);
        Ok(())
    }

    #[test]
    fn test_bitxor_assign_ref_u8() -> Result<(), crate::BufferError> {
        let mut buf = crate::Buffer::from_slice(&[0b1010, 0b1100])?;
        buf ^= &0b1111u8;
        assert_eq!(&*buf, &[0b0101, 0b0011]);
        Ok(())
    }

    #[test]
    fn test_shl_assign_u8() -> Result<(), crate::BufferError> {
        let mut buf = crate::Buffer::from_slice(&[1, 2, 3])?;
        buf <<= 2u8;
        assert_eq!(&*buf, &[4, 8, 12]);
        Ok(())
    }

    #[test]
    fn test_shr_assign_u8() -> Result<(), crate::BufferError> {
        let mut buf = crate::Buffer::from_slice(&[4, 8, 12])?;
        buf >>= 2u8;
        assert_eq!(&*buf, &[1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_different_lengths_slice() -> Result<(), crate::BufferError> {
        let mut buf = crate::Buffer::from_slice(&[0b1010, 0b1100, 0b1111])?;
        let rhs = [0b1100, 0b1010];
        buf &= &rhs[..];
        assert_eq!(&*buf, &[0b1000, 0b1000, 0b1111]);
        Ok(())
    }
}

use core::str::from_utf8;

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::VMResult;
use crate::types::Pointer;

pub struct ByteReader<'a> {
    pub pc: Pointer,
    program: &'a [u8],
}

impl<'a> ByteReader<'a> {
    pub fn new(pc: Pointer, program: &'a [u8]) -> Self {
        Self { pc, program }
    }

    pub fn parse_u8(&self) -> VMResult<u8> {
        let number = *self
            .program
            .get(self.pc + 1)
            .ok_or("Out of program for u8")?;

        Ok(number)
    }

    pub fn parse_u16(&self) -> VMResult<u16> {
        let bytes = self
            .program
            .get(self.pc + 1..self.pc + 3)
            .ok_or("Out of program for u16")?;

        Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn parse_u32(&self) -> VMResult<u32> {
        let bytes = self
            .program
            .get(self.pc + 1..self.pc + 5)
            .ok_or("Out of program for u32")?;

        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn parse_i8(&self) -> VMResult<i8> {
        Ok(self.parse_u8()? as i8)
    }

    pub fn parse_i16(&self) -> VMResult<i16> {
        Ok(self.parse_u16()? as i16)
    }

    pub fn parse_i32(&self) -> VMResult<i32> {
        Ok(self.parse_u32()? as i32)
    }

    pub fn parse_i64(&self) -> VMResult<i64> {
        Ok(self.parse_u64()? as i64)
    }

    pub fn parse_u64(&self) -> VMResult<u64> {
        let bytes = self
            .program
            .get(self.pc + 1..self.pc + 9)
            .ok_or("Out of program for u64")?;

        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn parse_string(&self, size: usize) -> VMResult<&'a str> {
        let bytes = self
            .program
            .get(self.pc..self.pc + size)
            .ok_or("Out of program for string")?;

        let string = from_utf8(bytes).map_err(|_| "Invalid UTF-8")?;
        Ok(string)
    }
}

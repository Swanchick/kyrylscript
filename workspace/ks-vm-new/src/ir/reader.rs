#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::VMResult;
use crate::types::Pointer;

pub struct Reader<'a> {
    pc: Pointer,
    program: &'a [u8],
}

impl<'a> Reader<'a> {
    pub fn new(pc: Pointer, program: &'a [u8]) -> Self {
        Self { pc, program }
    }

    pub fn parse_u8(self) -> VMResult<u64> {
        let number = *self.program.get(self.pc + 1).ok_or("Out of program")? as u64;
        Ok(number)
    }

    pub fn parse_u16(self) -> VMResult<u64> {
        let bytes = self
            .program
            .get(self.pc + 1..self.pc + 3)
            .ok_or("Out of program")?;

        Ok(u16::from_le_bytes(bytes.try_into().unwrap()) as u64)
    }

    pub fn parse_u32(self) -> VMResult<u64> {
        let bytes = self
            .program
            .get(self.pc + 1..self.pc + 5)
            .ok_or("Out of program")?;

        Ok(u32::from_le_bytes(bytes.try_into().unwrap()) as u64)
    }

    pub fn parse_u64(self) -> VMResult<u64> {
        let bytes = self
            .program
            .get(self.pc + 1..self.pc + 9)
            .ok_or("Out of program")?;

        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }
}

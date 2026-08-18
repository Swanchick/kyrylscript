const BYTE: isize = 1;
const WORD: isize = 2;
const DWORD: isize = 4;
const QWORD: isize = 8;
const INSTRUCTION: isize = 1;

pub enum DataSize64 {
    Byte,  // 1 byte
    Word,  // 2 bytes
    DWord, // 4 bytes
    QWord, // 8 bytes
}

impl DataSize64 {
    pub fn instruction_size(self) -> isize {
        match self {
            Self::Byte => INSTRUCTION + BYTE,
            Self::Word => INSTRUCTION + WORD,
            Self::DWord => INSTRUCTION + DWORD,
            Self::QWord => INSTRUCTION + QWORD,
        }
    }
}

pub enum DataSize32 {
    Byte,  // 1 byte
    Word,  // 2 bytes
    DWord, // 4 bytes
}

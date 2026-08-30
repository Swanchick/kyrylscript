pub const BYTE_SIZE: usize = 8;

// Arithmetic (0x00-0x0F)
pub const ADD: u8 = 0x01;
pub const SUB: u8 = 0x02;
pub const MUL: u8 = 0x03;
pub const DIV: u8 = 0x04;
pub const INC: u8 = 0x05;
pub const DEC: u8 = 0x06;

// Comparison (0x10-0x1F)
pub const EQ: u8 = 0x10;
pub const NE: u8 = 0x11;
pub const GT: u8 = 0x12;
pub const GE: u8 = 0x13;
pub const LT: u8 = 0x14; // LITHUANIA LET'S GOOOOOO
pub const LE: u8 = 0x15;

// Logic (0x20-0x2F)
pub const AND: u8 = 0x20;
pub const OR: u8 = 0x21;
pub const NOT: u8 = 0x22;

// Branching (0x30-0x3F)
pub const RET: u8 = 0x30;
pub const JZ: u8 = 0x31; // <u32>
pub const JNZ: u8 = 0x32; // <u32>
pub const JMP: u8 = 0x33; // <u32>

// Stack (0x40-0x4F)
pub const CPY: u8 = 0x40;
pub const CLR: u8 = 0x41;
pub const FREE: u8 = 0x42; // <u32>
pub const CALL: u8 = 0x43; // <u32>
pub const NCALL: u8 = 0x44; // <u32>, <u32>

// MEMORY (0x50-0x5F)
pub const STR: u8 = 0x50;
pub const ASN: u8 = 0x51;
pub const ASV: u8 = 0x52; // <u32>
pub const ASC: u8 = 0x53;
pub const LDV: u8 = 0x54; // <u32>
pub const LDCP: u8 = 0x57; // <u32>
pub const LDFC: u8 = 0x58;
pub const LEN: u8 = 0x59;

// Standard constants (0x60-0x6F)
pub const LDI: u8 = 0x60; // <i64>
pub const LDF: u8 = 0x61; // <f64>
pub const LDS: u8 = 0x62; // <u8>, <&str>
pub const LBT: u8 = 0x63;
pub const LBF: u8 = 0x64;
pub const LDN: u8 = 0x65;
pub const LDFN: u8 = 0x66; // <u32>
pub const LDC: u8 = 0x67;

// Small sized constants (0x70-0x7F)
pub const LDI8: u8 = 0x70; // <u8>
pub const LDI16: u8 = 0x71; // <u16>
pub const LDI32: u8 = 0x72; // <u32>
pub const LDC8: u8 = 0x73; // <u8>
pub const LDC16: u8 = 0x74; // <u16>

// Small sized memory (0x80-0x08F)
pub const LDV8: u8 = 0x80; // <u8>
pub const LDV16: u8 = 0x81; // <u16>
pub const ASV8: u8 = 0x82; // <u8>
pub const ASV16: u8 = 0x83; // <u16>
pub const LDCP8: u8 = 0x84; // <u8>
pub const LDCP16: u8 = 0x85; // <u16>

// Small sized stack (0x90-0x9F)
pub const FREE8: u8 = 0x90; // <u8>
pub const FREE16: u8 = 0x91; // <u16>
pub const CALL8: u8 = 0x92; // <u8>
pub const CALL16: u8 = 0x93; // <u16>

// Small sized branching (0xA0-0xAF)
pub const JZ8: u8 = 0xA0; // <u8>
pub const JZ16: u8 = 0xA1; // <u16>
pub const JNZ8: u8 = 0xA2; // <u8>
pub const JNZ16: u8 = 0xA3; // <u16>
pub const JMP8: u8 = 0xA4; // <u8>
pub const JMP16: u8 = 0xA5; // <u16>

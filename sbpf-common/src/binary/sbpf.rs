// SBPF-specific binary structures and utilities

/// SBPF instruction opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Load/Store
    LdAbs,
    LdInd,
    Ldx,
    St,
    Stx,
    // ALU
    Add,
    Sub,
    Mul,
    Div,
    Or,
    And,
    Lsh,
    Rsh,
    Mod,
    Xor,
    Mov,
    Arsh,
    // Jump
    Ja,
    Jeq,
    Jgt,
    Jge,
    Jlt,
    Jle,
    Jset,
    Jne,
    Jsgt,
    Jsge,
    Jslt,
    Jsle,
    Call,
    Exit,
    // Misc
    Unknown(u8),
}

impl Opcode {
    /// Get opcode from instruction byte
    pub fn from_u8(byte: u8) -> Self {
        match byte & 0xf0 {
            0x00 => Opcode::LdAbs,
            0x10 => Opcode::LdInd,
            0x20 => Opcode::Ldx,
            0x30 => Opcode::St,
            0x40 => Opcode::Stx,
            0x50 => Opcode::Add,
            0x60 => Opcode::Sub,
            0x70 => Opcode::Mul,
            0x80 => Opcode::Div,
            0x90 => Opcode::Or,
            0xa0 => Opcode::And,
            0xb0 => Opcode::Lsh,
            0xc0 => Opcode::Rsh,
            0xd0 => Opcode::Mod,
            0xe0 => Opcode::Xor,
            0xf0 => match byte & 0x0f {
                0x00 => Opcode::Mov,
                0x01 => Opcode::Arsh,
                0x02 => Opcode::Ja,
                0x03 => Opcode::Jeq,
                0x04 => Opcode::Jgt,
                0x05 => Opcode::Jge,
                0x06 => Opcode::Jlt,
                0x07 => Opcode::Jle,
                0x08 => Opcode::Jset,
                0x09 => Opcode::Jne,
                0x0a => Opcode::Jsgt,
                0x0b => Opcode::Jsge,
                0x0c => Opcode::Jslt,
                0x0d => Opcode::Jsle,
                0x0e => Opcode::Call,
                0x0f => Opcode::Exit,
                _ => Opcode::Unknown(byte),
            },
            _ => Opcode::Unknown(byte),
        }
    }
}


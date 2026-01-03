use crate::binary::SbpfVersion;
use crate::binary::sbpf::Opcode;

/// A decoded SBPF instruction
#[derive(Debug, Clone)]
pub struct Instruction {
    /// Instruction address/offset
    pub address: u64,
    /// Opcode
    pub opcode: Opcode,
    /// Destination register (0-10)
    pub dst_reg: u8,
    /// Source register (0-10)
    pub src_reg: u8,
    /// Immediate value (if applicable)
    pub imm: i64,
    /// Offset (for memory operations)
    pub off: i16,
    /// Instruction size in bytes
    pub size: usize,
}

/// Instruction category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstructionCategory {
    LoadStore,
    Arithmetic,
    Logic,
    ControlFlow,
    Misc,
}

impl Instruction {
    /// Get the category of this instruction
    pub fn category(&self) -> InstructionCategory {
        match self.opcode {
            Opcode::LdAbs | Opcode::LdInd | Opcode::Ldx | Opcode::St | Opcode::Stx => {
                InstructionCategory::LoadStore
            }
            Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div | Opcode::Mod | Opcode::Mov | Opcode::Arsh => {
                InstructionCategory::Arithmetic
            }
            Opcode::Or | Opcode::And | Opcode::Xor | Opcode::Lsh | Opcode::Rsh => {
                InstructionCategory::Logic
            }
            Opcode::Ja | Opcode::Jeq | Opcode::Jgt | Opcode::Jge | Opcode::Jlt | Opcode::Jle
            | Opcode::Jset | Opcode::Jne | Opcode::Jsgt | Opcode::Jsge | Opcode::Jslt
            | Opcode::Jsle | Opcode::Call | Opcode::Exit => {
                InstructionCategory::ControlFlow
            }
            Opcode::Unknown(_) => InstructionCategory::Misc,
        }
    }
    
    /// Check if this is a syscall instruction
    pub fn is_syscall(&self) -> bool {
        matches!(self.opcode, Opcode::Call) && self.imm > 0
    }
    
    /// Get syscall number if this is a syscall
    pub fn syscall_number(&self) -> Option<u64> {
        if self.is_syscall() {
            Some(self.imm as u64)
        } else {
            None
        }
    }
}

/// Decode SBPF instructions from bytecode
pub struct InstructionDecoder {
    #[allow(dead_code)]
    version: Option<SbpfVersion>,
}

impl InstructionDecoder {
    /// Create a new decoder
    pub fn new(version: Option<SbpfVersion>) -> Self {
        Self { version }
    }
    
    /// Decode all instructions from bytecode
    pub fn decode_all(&self, bytecode: &[u8]) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        let mut offset = 0;
        
        while offset < bytecode.len() {
            if let Some(inst) = self.decode_at(bytecode, offset) {
                offset += inst.size;
                instructions.push(inst);
            } else {
                // Skip invalid instruction
                offset += 8; // SBPF instructions are typically 8 bytes
            }
        }
        
        instructions
    }
    
    /// Decode instruction at a specific offset
    pub fn decode_at(&self, bytecode: &[u8], offset: usize) -> Option<Instruction> {
        if offset + 8 > bytecode.len() {
            return None;
        }
        
        // SBPF instructions are 8 bytes
        let bytes = &bytecode[offset..offset + 8];
        
        // Parse instruction format:
        // [opcode:8] [dst_reg:4] [src_reg:4] [off:16] [imm:32]
        let opcode_byte = bytes[0];
        let opcode = Opcode::from_u8(opcode_byte);
        
        let dst_reg = (bytes[1] & 0x0f) as u8;
        let src_reg = ((bytes[1] >> 4) & 0x0f) as u8;
        
        let off = i16::from_le_bytes([bytes[2], bytes[3]]);
        let imm = i32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as i64;
        
        Some(Instruction {
            address: offset as u64,
            opcode,
            dst_reg,
            src_reg,
            imm,
            off,
            size: 8,
        })
    }
}

/// Detect SBPF version from instruction patterns
pub fn detect_version_from_instructions(instructions: &[Instruction]) -> Option<SbpfVersion> {
    // Simplified version detection based on instruction usage
    // This is a placeholder - real detection would be more sophisticated
    if instructions.is_empty() {
        return None;
    }
    
    // Check for version-specific features
    // For now, default to V0
    Some(SbpfVersion::V0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_instruction() {
        let decoder = InstructionDecoder::new(None);
        // Create a simple instruction: mov r1, 42
        let mut bytes = vec![0u8; 8];
        bytes[0] = 0xb7; // mov opcode
        bytes[1] = 0x01; // dst=1, src=0
        bytes[4..8].copy_from_slice(&42i32.to_le_bytes());
        
        let inst = decoder.decode_at(&bytes, 0).unwrap();
        assert_eq!(inst.dst_reg, 1);
        assert_eq!(inst.imm, 42);
    }
}

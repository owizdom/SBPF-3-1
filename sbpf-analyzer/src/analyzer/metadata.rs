use sbpf_common::binary::{SbpfBinary, SbpfVersion};
use sbpf_common::instruction::Instruction;

/// Metadata extracted from the binary
#[derive(Debug, Clone)]
pub struct Metadata {
    /// Total program size in bytes
    pub program_size: usize,
    /// Entry point address
    pub entry_point: u64,
    /// Total number of instructions
    pub instruction_count: usize,
    /// Detected SBPF version
    pub version: Option<SbpfVersion>,
    /// Number of metadata sections
    pub metadata_sections: usize,
}

/// Extract metadata from binary and instructions
pub fn extract(binary: &SbpfBinary, instructions: &[Instruction]) -> Metadata {
    Metadata {
        program_size: binary.bytecode.len(),
        entry_point: binary.entry_point,
        instruction_count: instructions.len(),
        version: binary.version,
        metadata_sections: binary.metadata.len(),
    }
}


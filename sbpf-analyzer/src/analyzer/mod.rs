pub mod metadata;
pub mod instructions;
pub mod syscalls;

use crate::analyzer::metadata::Metadata;
use crate::analyzer::instructions::InstructionStats;
use crate::analyzer::syscalls::SyscallInfo;
use sbpf_common::binary::SbpfBinary;
use sbpf_common::instruction::InstructionDecoder;

/// Analysis results for an SBPF binary
#[derive(Debug, Clone)]
pub struct Analysis {
    pub metadata: Metadata,
    pub instruction_stats: InstructionStats,
    pub syscall_info: SyscallInfo,
}

/// Analyze an SBPF binary
pub fn analyze(binary: &SbpfBinary) -> Analysis {
    let decoder = InstructionDecoder::new(binary.version);
    let instructions = decoder.decode_all(&binary.bytecode);
    
    Analysis {
        metadata: metadata::extract(binary, &instructions),
        instruction_stats: instructions::analyze(&instructions),
        syscall_info: syscalls::analyze(&instructions),
    }
}


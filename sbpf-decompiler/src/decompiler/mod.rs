pub mod disasm;
pub mod lifter;
pub mod pseudocode;

use sbpf_common::binary::SbpfBinary;
use sbpf_common::instruction::InstructionDecoder;
use crate::decompiler::disasm::disassemble;
use crate::decompiler::lifter::lift_to_ir;
use crate::decompiler::pseudocode::generate_pseudocode;

/// Decompile an SBPF binary to Rust-like pseudocode
pub fn decompile(binary: &SbpfBinary) -> String {
    let decoder = InstructionDecoder::new(binary.version);
    let instructions = decoder.decode_all(&binary.bytecode);
    
    // Disassemble
    let assembly = disassemble(&instructions);
    
    // Lift to IR
    let ir = lift_to_ir(&instructions);
    
    // Generate pseudocode
    generate_pseudocode(&ir, &assembly)
}


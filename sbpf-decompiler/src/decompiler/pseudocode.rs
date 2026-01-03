use crate::decompiler::lifter::{IR, IRInstruction};
use crate::decompiler::disasm::DisassembledInstruction;

/// Generate Rust-like pseudocode from IR
pub fn generate_pseudocode(ir: &IR, assembly: &[DisassembledInstruction]) -> String {
    let mut output = String::new();
    
    output.push_str("// Decompiled SBPF Program\n");
    output.push_str("// Generated pseudocode\n\n");
    
    // Generate function for entry point
    output.push_str("fn entry() -> Result<(), Error> {\n");
    
    // Generate code for each basic block
    for (idx, block) in ir.basic_blocks.iter().enumerate() {
        if idx > 0 {
            output.push_str(&format!("\n    // Basic block at 0x{:x}\n", block.address));
        }
        
        // Generate code for instructions in block
        for inst in &block.instructions {
            output.push_str(&format!("    {}", generate_instruction_code(inst)));
            output.push_str("\n");
        }
        
        // Handle control flow
        if !block.successors.is_empty() {
            if block.successors.len() == 1 {
                output.push_str(&format!("    // Jump to 0x{:x}\n", block.successors[0]));
            } else {
                output.push_str("    // Conditional jump\n");
            }
        }
    }
    
    output.push_str("}\n\n");
    
    // Add assembly comments for reference
    output.push_str("// Assembly reference:\n");
    for asm in assembly.iter().take(20) {
        output.push_str(&format!("// 0x{:x}: {} {}\n", asm.address, asm.mnemonic, asm.operands));
    }
    if assembly.len() > 20 {
        output.push_str(&format!("// ... ({} more instructions)\n", assembly.len() - 20));
    }
    
    output
}

fn generate_instruction_code(inst: &IRInstruction) -> String {
    match inst {
        IRInstruction::Load { dst, src, offset } => {
            format!("let r{} = *((r{} as *const u64).offset({}));", dst, src, offset / 8)
        }
        IRInstruction::Store { dst, src, offset } => {
            format!("*((r{} as *mut u64).offset({})) = r{};", dst, offset / 8, src)
        }
        IRInstruction::Arithmetic { op, dst, src, imm } => {
            if let Some(imm_val) = imm {
                format!("r{} = r{} {} {};", dst, src, op, imm_val)
            } else {
                format!("r{} = r{} {} r{};", dst, src, op, src)
            }
        }
        IRInstruction::Logic { op, dst, src, imm } => {
            if let Some(imm_val) = imm {
                format!("r{} = r{} {} {};", dst, src, op, imm_val)
            } else {
                format!("r{} = r{} {} r{};", dst, src, op, src)
            }
        }
        IRInstruction::Jump { cond, target } => {
            if let Some(cond_str) = cond {
                format!("if r{} {} r{} {{ goto 0x{:x}; }}", 0, cond_str, 0, target)
            } else {
                format!("goto 0x{:x};", target)
            }
        }
        IRInstruction::Call { target, is_syscall } => {
            if *is_syscall {
                format!("syscall({});", target)
            } else {
                format!("call(0x{:x});", *target as u64)
            }
        }
        IRInstruction::Return => {
            "return;".to_string()
        }
        IRInstruction::Exit => {
            "exit();".to_string()
        }
    }
}


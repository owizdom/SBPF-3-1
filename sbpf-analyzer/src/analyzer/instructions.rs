use sbpf_common::instruction::{Instruction, InstructionCategory};
use std::collections::HashMap;

/// Statistics about instructions
#[derive(Debug, Clone)]
pub struct InstructionStats {
    /// Total instruction count
    pub total: usize,
    /// Count by category
    pub by_category: HashMap<InstructionCategory, usize>,
    /// Count by opcode
    pub by_opcode: HashMap<String, usize>,
    /// Control flow information
    pub control_flow: ControlFlowInfo,
}

/// Control flow information
#[derive(Debug, Clone)]
pub struct ControlFlowInfo {
    /// Number of jump instructions
    pub jumps: usize,
    /// Number of call instructions
    pub calls: usize,
    /// Number of exit instructions
    pub exits: usize,
    /// Unique jump targets
    pub jump_targets: Vec<u64>,
}

/// Analyze instruction statistics
pub fn analyze(instructions: &[Instruction]) -> InstructionStats {
    let mut by_category: HashMap<InstructionCategory, usize> = HashMap::new();
    let mut by_opcode: HashMap<String, usize> = HashMap::new();
    let mut jump_targets = Vec::new();
    let mut jumps = 0;
    let mut calls = 0;
    let mut exits = 0;
    
    for inst in instructions {
        // Count by category
        let category = inst.category();
        *by_category.entry(category).or_insert(0) += 1;
        
        // Count by opcode
        let opcode_name = format!("{:?}", inst.opcode);
        *by_opcode.entry(opcode_name).or_insert(0) += 1;
        
        // Analyze control flow
        match inst.category() {
            InstructionCategory::ControlFlow => {
                match inst.opcode {
                    sbpf_common::binary::sbpf::Opcode::Call => {
                        calls += 1;
                    }
                    sbpf_common::binary::sbpf::Opcode::Exit => {
                        exits += 1;
                    }
                    _ => {
                        jumps += 1;
                        // Calculate jump target
                        let target = (inst.address as i64 + inst.imm + 1) as u64;
                        if !jump_targets.contains(&target) {
                            jump_targets.push(target);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    
    jump_targets.sort();
    
    InstructionStats {
        total: instructions.len(),
        by_category,
        by_opcode,
        control_flow: ControlFlowInfo {
            jumps,
            calls,
            exits,
            jump_targets,
        },
    }
}


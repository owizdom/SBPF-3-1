use sbpf_common::instruction::Instruction;
use std::collections::HashMap;

/// Intermediate representation for decompilation
#[derive(Debug, Clone)]
pub struct IR {
    pub basic_blocks: Vec<BasicBlock>,
    #[allow(dead_code)]
    pub functions: Vec<Function>,
}

/// Basic block in the IR
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub address: u64,
    pub instructions: Vec<IRInstruction>,
    pub successors: Vec<u64>,
}

/// Function in the IR
#[derive(Debug, Clone)]
pub struct Function {
    #[allow(dead_code)]
    pub address: u64,
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub basic_blocks: Vec<u64>,
}

/// IR instruction representation
#[derive(Debug, Clone)]
pub enum IRInstruction {
    Load { dst: u8, src: u8, offset: i16 },
    Store { dst: u8, src: u8, offset: i16 },
    Arithmetic { op: String, dst: u8, src: u8, imm: Option<i64> },
    Logic { op: String, dst: u8, src: u8, imm: Option<i64> },
    Jump { cond: Option<String>, target: u64 },
    Call { target: i64, is_syscall: bool },
    Return,
    Exit,
}

/// Lift instructions to intermediate representation
pub fn lift_to_ir(instructions: &[Instruction]) -> IR {
    let mut basic_blocks = Vec::new();
    let mut functions = Vec::new();
    let mut block_map: HashMap<u64, usize> = HashMap::new();
    
    // Identify basic blocks (simplified - starts at entry and after jumps)
    let mut block_starts = vec![0u64];
    for (i, inst) in instructions.iter().enumerate() {
        if matches!(inst.category(), sbpf_common::instruction::InstructionCategory::ControlFlow) {
            if let Some(target) = calculate_jump_target(inst) {
                if !block_starts.contains(&target) {
                    block_starts.push(target);
                }
            }
            if i + 1 < instructions.len() {
                block_starts.push(instructions[i + 1].address);
            }
        }
    }
    block_starts.sort();
    block_starts.dedup();
    
    // Create basic blocks
    for (idx, &start_addr) in block_starts.iter().enumerate() {
        let end_addr = if idx + 1 < block_starts.len() {
            block_starts[idx + 1]
        } else {
            instructions.last().map(|i| i.address + i.size as u64).unwrap_or(start_addr)
        };
        
        let mut block_instructions = Vec::new();
        let mut successors = Vec::new();
        
        for inst in instructions {
            if inst.address >= start_addr && inst.address < end_addr {
                block_instructions.push(lift_instruction(inst));
                
                // Track successors
                if let Some(target) = calculate_jump_target(inst) {
                    successors.push(target);
                }
            }
        }
        
        let block_idx = basic_blocks.len();
        block_map.insert(start_addr, block_idx);
        
        basic_blocks.push(BasicBlock {
            address: start_addr,
            instructions: block_instructions,
            successors,
        });
    }
    
    // Identify functions (simplified - entry point and call targets)
    if !instructions.is_empty() {
        functions.push(Function {
            address: instructions[0].address,
            name: "entry".to_string(),
            basic_blocks: vec![0],
        });
    }
    
    IR {
        basic_blocks,
        functions,
    }
}

fn lift_instruction(inst: &Instruction) -> IRInstruction {
    match inst.opcode {
        sbpf_common::binary::sbpf::Opcode::Ldx => {
            IRInstruction::Load {
                dst: inst.dst_reg,
                src: inst.src_reg,
                offset: inst.off,
            }
        }
        sbpf_common::binary::sbpf::Opcode::Stx => {
            IRInstruction::Store {
                dst: inst.dst_reg,
                src: inst.src_reg,
                offset: inst.off,
            }
        }
        sbpf_common::binary::sbpf::Opcode::Add | sbpf_common::binary::sbpf::Opcode::Sub
        | sbpf_common::binary::sbpf::Opcode::Mul | sbpf_common::binary::sbpf::Opcode::Div
        | sbpf_common::binary::sbpf::Opcode::Mod | sbpf_common::binary::sbpf::Opcode::Mov => {
            IRInstruction::Arithmetic {
                op: format!("{:?}", inst.opcode).to_lowercase(),
                dst: inst.dst_reg,
                src: inst.src_reg,
                imm: if inst.imm != 0 { Some(inst.imm) } else { None },
            }
        }
        sbpf_common::binary::sbpf::Opcode::Or | sbpf_common::binary::sbpf::Opcode::And
        | sbpf_common::binary::sbpf::Opcode::Xor | sbpf_common::binary::sbpf::Opcode::Lsh
        | sbpf_common::binary::sbpf::Opcode::Rsh => {
            IRInstruction::Logic {
                op: format!("{:?}", inst.opcode).to_lowercase(),
                dst: inst.dst_reg,
                src: inst.src_reg,
                imm: if inst.imm != 0 { Some(inst.imm) } else { None },
            }
        }
        sbpf_common::binary::sbpf::Opcode::Call => {
            IRInstruction::Call {
                target: inst.imm,
                is_syscall: inst.is_syscall(),
            }
        }
        sbpf_common::binary::sbpf::Opcode::Exit => {
            IRInstruction::Exit
        }
        _ => {
            // For jumps, we'll handle them in the basic block structure
            if let Some(target) = calculate_jump_target(inst) {
                let cond = match inst.opcode {
                    sbpf_common::binary::sbpf::Opcode::Jeq => Some("==".to_string()),
                    sbpf_common::binary::sbpf::Opcode::Jne => Some("!=".to_string()),
                    sbpf_common::binary::sbpf::Opcode::Jgt => Some(">".to_string()),
                    sbpf_common::binary::sbpf::Opcode::Jge => Some(">=".to_string()),
                    sbpf_common::binary::sbpf::Opcode::Jlt => Some("<".to_string()),
                    sbpf_common::binary::sbpf::Opcode::Jle => Some("<=".to_string()),
                    _ => None,
                };
                IRInstruction::Jump { cond, target }
            } else {
                IRInstruction::Return
            }
        }
    }
}

fn calculate_jump_target(inst: &Instruction) -> Option<u64> {
    match inst.category() {
        sbpf_common::instruction::InstructionCategory::ControlFlow => {
            if matches!(inst.opcode, sbpf_common::binary::sbpf::Opcode::Ja) {
                Some((inst.address as i64 + inst.imm + 1) as u64)
            } else if matches!(
                inst.opcode,
                sbpf_common::binary::sbpf::Opcode::Jeq | sbpf_common::binary::sbpf::Opcode::Jne
                | sbpf_common::binary::sbpf::Opcode::Jgt | sbpf_common::binary::sbpf::Opcode::Jge
                | sbpf_common::binary::sbpf::Opcode::Jlt | sbpf_common::binary::sbpf::Opcode::Jle
            ) {
                Some((inst.address as i64 + inst.imm + 1) as u64)
            } else {
                None
            }
        }
        _ => None,
    }
}


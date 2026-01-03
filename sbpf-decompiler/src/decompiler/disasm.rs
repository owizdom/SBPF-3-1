use sbpf_common::instruction::Instruction;
use sbpf_common::binary::sbpf::Opcode;

/// Disassembled instruction
#[derive(Debug, Clone)]
pub struct DisassembledInstruction {
    pub address: u64,
    pub mnemonic: String,
    pub operands: String,
    #[allow(dead_code)]
    pub comment: Option<String>,
}

/// Disassemble instructions to assembly format
pub fn disassemble(instructions: &[Instruction]) -> Vec<DisassembledInstruction> {
    instructions.iter().map(|inst| {
        let mnemonic = opcode_to_mnemonic(inst.opcode);
        let operands = format_operands(inst);
        let comment = format_comment(inst);
        
        DisassembledInstruction {
            address: inst.address,
            mnemonic,
            operands,
            comment,
        }
    }).collect()
}

fn opcode_to_mnemonic(opcode: Opcode) -> String {
    match opcode {
        Opcode::LdAbs => "ldabs".to_string(),
        Opcode::LdInd => "ldind".to_string(),
        Opcode::Ldx => "ldx".to_string(),
        Opcode::St => "st".to_string(),
        Opcode::Stx => "stx".to_string(),
        Opcode::Add => "add".to_string(),
        Opcode::Sub => "sub".to_string(),
        Opcode::Mul => "mul".to_string(),
        Opcode::Div => "div".to_string(),
        Opcode::Or => "or".to_string(),
        Opcode::And => "and".to_string(),
        Opcode::Lsh => "lsh".to_string(),
        Opcode::Rsh => "rsh".to_string(),
        Opcode::Mod => "mod".to_string(),
        Opcode::Xor => "xor".to_string(),
        Opcode::Mov => "mov".to_string(),
        Opcode::Arsh => "arsh".to_string(),
        Opcode::Ja => "ja".to_string(),
        Opcode::Jeq => "jeq".to_string(),
        Opcode::Jgt => "jgt".to_string(),
        Opcode::Jge => "jge".to_string(),
        Opcode::Jlt => "jlt".to_string(),
        Opcode::Jle => "jle".to_string(),
        Opcode::Jset => "jset".to_string(),
        Opcode::Jne => "jne".to_string(),
        Opcode::Jsgt => "jsgt".to_string(),
        Opcode::Jsge => "jsge".to_string(),
        Opcode::Jslt => "jslt".to_string(),
        Opcode::Jsle => "jsle".to_string(),
        Opcode::Call => "call".to_string(),
        Opcode::Exit => "exit".to_string(),
        Opcode::Unknown(b) => format!("unknown_{:02x}", b),
    }
}

fn format_operands(inst: &Instruction) -> String {
    match inst.opcode {
        Opcode::Call => {
            if inst.is_syscall() {
                format!("syscall_{}", inst.imm)
            } else {
                format!("{}", inst.imm)
            }
        }
        Opcode::Ja | Opcode::Jeq | Opcode::Jgt | Opcode::Jge | Opcode::Jlt | Opcode::Jle
        | Opcode::Jset | Opcode::Jne | Opcode::Jsgt | Opcode::Jsge | Opcode::Jslt | Opcode::Jsle => {
            let target = (inst.address as i64 + inst.imm + 1) as u64;
            format!("r{}, r{}, 0x{:x}", inst.dst_reg, inst.src_reg, target)
        }
        Opcode::LdAbs | Opcode::LdInd => {
            format!("r{}, [{}]", inst.dst_reg, inst.imm)
        }
        Opcode::Ldx => {
            format!("r{}, [r{}+{}]", inst.dst_reg, inst.src_reg, inst.off)
        }
        Opcode::St | Opcode::Stx => {
            format!("[r{}+{}], r{}", inst.dst_reg, inst.off, inst.src_reg)
        }
        Opcode::Mov => {
            format!("r{}, {}", inst.dst_reg, inst.imm)
        }
        Opcode::Exit => {
            format!("r{}", inst.dst_reg)
        }
        _ => {
            if inst.imm != 0 {
                format!("r{}, r{}, {}", inst.dst_reg, inst.src_reg, inst.imm)
            } else {
                format!("r{}, r{}", inst.dst_reg, inst.src_reg)
            }
        }
    }
}

fn format_comment(inst: &Instruction) -> Option<String> {
    if inst.is_syscall() {
        Some(format!("syscall {}", inst.imm))
    } else {
        None
    }
}


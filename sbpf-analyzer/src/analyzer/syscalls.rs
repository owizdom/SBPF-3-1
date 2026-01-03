use sbpf_common::instruction::Instruction;
use std::collections::HashMap;

/// Syscall information
#[derive(Debug, Clone)]
pub struct SyscallInfo {
    /// Total syscall count
    pub total: usize,
    /// Syscall frequency map (syscall number -> count)
    pub frequency: HashMap<u64, usize>,
    /// List of unique syscall numbers
    pub syscalls: Vec<u64>,
}

/// Analyze syscalls in instructions
pub fn analyze(instructions: &[Instruction]) -> SyscallInfo {
    let mut frequency: HashMap<u64, usize> = HashMap::new();
    let mut syscalls = Vec::new();
    
    for inst in instructions {
        if let Some(syscall_num) = inst.syscall_number() {
            *frequency.entry(syscall_num).or_insert(0) += 1;
            if !syscalls.contains(&syscall_num) {
                syscalls.push(syscall_num);
            }
        }
    }
    
    syscalls.sort();
    
    SyscallInfo {
        total: frequency.values().sum(),
        frequency,
        syscalls,
    }
}

/// Map syscall number to name (Solana syscalls)
pub fn syscall_name(num: u64) -> Option<&'static str> {
    match num {
        0 => Some("sol_log"),
        1 => Some("sol_invoke"),
        2 => Some("sol_invoke_signed"),
        3 => Some("sol_create_account"),
        4 => Some("sol_assign"),
        5 => Some("sol_transfer"),
        6 => Some("sol_get_account_data_len"),
        7 => Some("sol_get_account_data"),
        8 => Some("sol_set_account_data"),
        9 => Some("sol_get_clock_sysvar"),
        10 => Some("sol_get_rent_sysvar"),
        11 => Some("sol_get_clock_sysvar"),
        12 => Some("sol_memcpy"),
        13 => Some("sol_memcmp"),
        14 => Some("sol_memset"),
        15 => Some("sol_invoke_signed"),
        _ => None,
    }
}


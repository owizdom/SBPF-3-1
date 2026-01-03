use crate::analyzer::Analysis;
use crate::output::OutputFormatter;

pub struct TextFormatter;

impl OutputFormatter for TextFormatter {
    fn format(&self, analysis: &Analysis) -> String {
        let mut output = String::new();
        
        // Header
        output.push_str("=== SBPF Binary Analysis ===\n\n");
        
        // Metadata
        output.push_str("## Metadata\n");
        output.push_str(&format!("Program Size: {} bytes\n", analysis.metadata.program_size));
        output.push_str(&format!("Entry Point: 0x{:x}\n", analysis.metadata.entry_point));
        output.push_str(&format!("Instruction Count: {}\n", analysis.metadata.instruction_count));
        if let Some(version) = analysis.metadata.version {
            output.push_str(&format!("SBPF Version: {:?}\n", version));
        } else {
            output.push_str("SBPF Version: Unknown\n");
        }
        output.push_str(&format!("Metadata Sections: {}\n", analysis.metadata.metadata_sections));
        output.push_str("\n");
        
        // Instruction Statistics
        output.push_str("## Instruction Statistics\n");
        output.push_str(&format!("Total Instructions: {}\n", analysis.instruction_stats.total));
        output.push_str("\nBy Category:\n");
        for (category, count) in &analysis.instruction_stats.by_category {
            output.push_str(&format!("  {:?}: {}\n", category, count));
        }
        output.push_str("\nBy Opcode:\n");
        let mut opcodes: Vec<_> = analysis.instruction_stats.by_opcode.iter().collect();
        opcodes.sort_by_key(|(_, count)| **count);
        opcodes.reverse();
        for (opcode, count) in opcodes.iter().take(10) {
            output.push_str(&format!("  {}: {}\n", opcode, count));
        }
        output.push_str("\n");
        
        // Control Flow
        output.push_str("## Control Flow\n");
        output.push_str(&format!("Jumps: {}\n", analysis.instruction_stats.control_flow.jumps));
        output.push_str(&format!("Calls: {}\n", analysis.instruction_stats.control_flow.calls));
        output.push_str(&format!("Exits: {}\n", analysis.instruction_stats.control_flow.exits));
        output.push_str(&format!("Unique Jump Targets: {}\n", analysis.instruction_stats.control_flow.jump_targets.len()));
        output.push_str("\n");
        
        // Syscalls
        output.push_str("## Syscalls\n");
        output.push_str(&format!("Total Syscalls: {}\n", analysis.syscall_info.total));
        if !analysis.syscall_info.syscalls.is_empty() {
            output.push_str("Syscall Usage:\n");
            for syscall_num in &analysis.syscall_info.syscalls {
                let count = analysis.syscall_info.frequency.get(syscall_num).unwrap_or(&0);
                let name = crate::analyzer::syscalls::syscall_name(*syscall_num)
                    .unwrap_or("unknown");
                output.push_str(&format!("  {} ({}): {}\n", name, syscall_num, count));
            }
        }
        output.push_str("\n");
        
        output
    }
}


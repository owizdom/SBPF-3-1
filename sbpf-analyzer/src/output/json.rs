use crate::analyzer::Analysis;
use crate::output::OutputFormatter;
use serde::Serialize;
use std::collections::HashMap;

pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format(&self, analysis: &Analysis) -> String {
        let json = AnalysisJson::from(analysis);
        serde_json::to_string_pretty(&json).unwrap_or_else(|_| "{}".to_string())
    }
}

#[derive(Serialize)]
struct AnalysisJson {
    metadata: MetadataJson,
    instructions: InstructionStatsJson,
    syscalls: SyscallInfoJson,
}

#[derive(Serialize)]
struct MetadataJson {
    program_size: usize,
    entry_point: String,
    instruction_count: usize,
    version: Option<String>,
    metadata_sections: usize,
}

#[derive(Serialize)]
struct InstructionStatsJson {
    total: usize,
    by_category: HashMap<String, usize>,
    by_opcode: HashMap<String, usize>,
    control_flow: ControlFlowInfoJson,
}

#[derive(Serialize)]
struct ControlFlowInfoJson {
    jumps: usize,
    calls: usize,
    exits: usize,
    jump_targets: Vec<String>,
}

#[derive(Serialize)]
struct SyscallInfoJson {
    total: usize,
    frequency: HashMap<String, usize>,
    syscalls: Vec<u64>,
}

impl From<&Analysis> for AnalysisJson {
    fn from(analysis: &Analysis) -> Self {
        AnalysisJson {
            metadata: MetadataJson {
                program_size: analysis.metadata.program_size,
                entry_point: format!("0x{:x}", analysis.metadata.entry_point),
                instruction_count: analysis.metadata.instruction_count,
                version: analysis.metadata.version.map(|v| format!("{:?}", v)),
                metadata_sections: analysis.metadata.metadata_sections,
            },
            instructions: InstructionStatsJson {
                total: analysis.instruction_stats.total,
                by_category: analysis.instruction_stats.by_category
                    .iter()
                    .map(|(k, v)| (format!("{:?}", k), *v))
                    .collect(),
                by_opcode: analysis.instruction_stats.by_opcode.clone(),
                control_flow: ControlFlowInfoJson {
                    jumps: analysis.instruction_stats.control_flow.jumps,
                    calls: analysis.instruction_stats.control_flow.calls,
                    exits: analysis.instruction_stats.control_flow.exits,
                    jump_targets: analysis.instruction_stats.control_flow.jump_targets
                        .iter()
                        .map(|t| format!("0x{:x}", t))
                        .collect(),
                },
            },
            syscalls: SyscallInfoJson {
                total: analysis.syscall_info.total,
                frequency: analysis.syscall_info.frequency
                    .iter()
                    .map(|(k, v)| (k.to_string(), *v))
                    .collect(),
                syscalls: analysis.syscall_info.syscalls.clone(),
            },
        }
    }
}


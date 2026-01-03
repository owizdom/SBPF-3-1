pub mod elf;
pub mod sbpf;

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Represents a parsed SBPF binary
#[derive(Debug, Clone)]
pub struct SbpfBinary {
    /// The raw bytecode
    pub bytecode: Vec<u8>,
    /// Entry point address
    pub entry_point: u64,
    /// SBPF version (if detectable)
    pub version: Option<SbpfVersion>,
    /// Metadata sections (if any)
    pub metadata: Vec<MetadataSection>,
}

/// SBPF version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SbpfVersion {
    V0,
    V1,
    V2,
    V3,
}

/// Metadata section from the binary
#[derive(Debug, Clone)]
pub struct MetadataSection {
    pub name: String,
    pub data: Vec<u8>,
}

/// Load and parse an SBPF binary from a file path
pub fn load_binary<P: AsRef<Path>>(path: P) -> Result<SbpfBinary> {
    let data = fs::read(path.as_ref())
        .with_context(|| format!("Failed to read file: {:?}", path.as_ref()))?;
    
    parse_binary(&data)
}

/// Parse binary data (ELF or raw bytecode)
pub fn parse_binary(data: &[u8]) -> Result<SbpfBinary> {
    // Try to parse as ELF first
    if let Ok(elf_binary) = elf::parse_elf(data) {
        return Ok(elf_binary);
    }
    
    // If not ELF, treat as raw bytecode
    Ok(SbpfBinary {
        bytecode: data.to_vec(),
        entry_point: 0,
        version: None,
        metadata: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_raw_bytecode() {
        let data = vec![0u8; 16];
        let binary = parse_binary(&data).unwrap();
        assert_eq!(binary.bytecode.len(), 16);
        assert_eq!(binary.entry_point, 0);
    }
}

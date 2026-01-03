use crate::binary::{MetadataSection, SbpfBinary, SbpfVersion};
use anyhow::{Context, Result};
use goblin::elf::Elf;
use goblin::Object;

/// Parse an ELF file and extract SBPF program data
pub fn parse_elf(data: &[u8]) -> Result<SbpfBinary> {
    let object = Object::parse(data)
        .context("Failed to parse ELF file")?;
    
    match object {
        Object::Elf(elf) => parse_elf_internal(&elf, data),
        _ => anyhow::bail!("Not an ELF file"),
    }
}

fn parse_elf_internal(elf: &Elf, data: &[u8]) -> Result<SbpfBinary> {
    let mut bytecode = Vec::new();
    let mut entry_point = 0;
    let mut metadata = Vec::new();
    
    // Extract .text section (bytecode)
    for section in &elf.section_headers {
        if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
            if name == ".text" {
                let start = section.sh_offset as usize;
                let end = start + section.sh_size as usize;
                if end <= data.len() {
                    bytecode = data[start..end].to_vec();
                    entry_point = section.sh_addr;
                }
            } else if name.starts_with(".metadata") || name.starts_with(".rodata") {
                // Extract metadata sections
                let start = section.sh_offset as usize;
                let end = start + section.sh_size as usize;
                if end <= data.len() {
                    metadata.push(MetadataSection {
                        name: name.to_string(),
                        data: data[start..end].to_vec(),
                    });
                }
            }
        }
    }
    
    if bytecode.is_empty() {
        anyhow::bail!("No .text section found in ELF file");
    }
    
    // Try to detect SBPF version from the binary
    let version = detect_version(&bytecode);
    
    Ok(SbpfBinary {
        bytecode,
        entry_point,
        version,
        metadata,
    })
}

/// Attempt to detect SBPF version from bytecode characteristics
fn detect_version(_bytecode: &[u8]) -> Option<SbpfVersion> {
    // This is a simplified version detection
    // In practice, version detection might require more sophisticated analysis
    // For now, we'll return None and let the instruction decoder help determine it
    None
}


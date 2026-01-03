mod decompiler;
mod output;

use anyhow::{Context, Result};
use clap::Parser;
use sbpf_common::binary::load_binary;
use crate::decompiler::decompile;

/// SBPF Decompiler
/// 
/// Decompiles SBPF binaries to Rust-like pseudocode for analysis
/// and reverse engineering.
#[derive(Parser, Debug)]
#[command(name = "sbpf-decompiler")]
#[command(version = "0.1.0")]
#[command(about = "Decompile SBPF binaries to pseudocode")]
struct Args {
    /// Path to the SBPF binary file
    #[arg(required = true)]
    binary: String,
    
    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<String>,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    if args.verbose {
        eprintln!("Loading binary: {}", args.binary);
    }
    
    let binary = load_binary(&args.binary)
        .with_context(|| format!("Failed to load binary: {}", args.binary))?;
    
    if args.verbose {
        eprintln!("Decompiling binary...");
    }
    
    let pseudocode = decompile(&binary);
    
    if let Some(output_path) = args.output {
        std::fs::write(&output_path, &pseudocode)
            .with_context(|| format!("Failed to write output to: {}", output_path))?;
        if args.verbose {
            eprintln!("Output written to: {}", output_path);
        }
    } else {
        println!("{}", pseudocode);
    }
    
    Ok(())
}

mod analyzer;
mod output;

use anyhow::{Context, Result};
use clap::Parser;
use sbpf_common::binary::load_binary;
use crate::analyzer::analyze;
use crate::output::{JsonFormatter, OutputFormatter, TextFormatter};

/// SBPF Binary Analyzer
/// 
/// Analyzes SBPF binaries and extracts metadata, instruction statistics,
/// syscall usage, and control flow information.
#[derive(Parser, Debug)]
#[command(name = "sbpf-analyzer")]
#[command(version = "0.1.0")]
#[command(about = "Analyze SBPF binaries")]
struct Args {
    /// Path to the SBPF binary file
    #[arg(required = true)]
    binary: String,
    
    /// Output format (text or json)
    #[arg(short, long, default_value = "text")]
    format: String,
    
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
        eprintln!("Analyzing binary...");
    }
    
    let analysis = analyze(&binary);
    
    let output = match args.format.as_str() {
        "json" => {
            let formatter = JsonFormatter;
            formatter.format(&analysis)
        }
        "text" | _ => {
            let formatter = TextFormatter;
            formatter.format(&analysis)
        }
    };
    
    println!("{}", output);
    
    Ok(())
}

# SBPF Tools

A comprehensive toolkit for analyzing and decompiling SBPF (Solana Berkeley Packet Filter) binaries.

This workspace contains two main tools:

- **sbpf-analyzer** - Analyze SBPF binaries and extract metadata, instruction statistics, syscall usage, and control flow information
- **sbpf-decompiler** - Decompile SBPF binaries to Rust-like pseudocode for security analysis and reverse engineering

Both tools share a common library (`sbpf-common`) for binary parsing and instruction decoding.

## Building

```bash
cargo build --release
```

## Usage

### sbpf-analyzer

Analyze an SBPF binary:

```bash
cargo run --bin sbpf-analyzer -- <binary-file>
```

Output as JSON:

```bash
cargo run --bin sbpf-analyzer -- --format json <binary-file>
```

### sbpf-decompiler

Decompile an SBPF binary:

```bash
cargo run --bin sbpf-decompiler -- <binary-file>
```

Save output to file:

```bash
cargo run --bin sbpf-decompiler -- --output output.rs <binary-file>
```

## Project Structure

```
sbpf-tools/
├── sbpf-common/      # Shared library for binary parsing and instruction decoding
├── sbpf-analyzer/    # Binary analyzer tool
└── sbpf-decompiler/  # Decompiler tool
```

## License

MIT OR Apache-2.0



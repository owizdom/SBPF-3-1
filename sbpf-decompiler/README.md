# SBPF Decompiler

A tool for decompiling SBPF (Solana Berkeley Packet Filter) binaries to Rust-like pseudocode.

## Features

- Disassemble SBPF bytecode to assembly instructions
- Lift instructions to intermediate representation (IR)
- Generate Rust-like pseudocode with:
  - Function definitions
  - Variable declarations (from register usage)
  - Control structures (if/else, loops)
  - Function calls and syscalls
  - Comments with original addresses

## Usage

```bash
sbpf-decompiler <binary-file> [OPTIONS]
```

### Options

- `-o, --output <file>` - Output file (default: stdout)
- `-v, --verbose` - Verbose output

### Examples

```bash
# Decompile to stdout
sbpf-decompiler program.so

# Save to file
sbpf-decompiler program.so --output output.rs

# Verbose output
sbpf-decompiler program.so --verbose
```

## Output Format

The decompiler generates Rust-like pseudocode with:

- Function definitions
- Register-based variable declarations
- Control flow structures
- Syscall invocations
- Assembly reference comments

## Limitations

- Variable names are inferred from register usage patterns
- Complex control flow may use goto statements
- Some optimizations may result in less readable code
- Type information is inferred and may be inaccurate

## License

MIT OR Apache-2.0


# SBPF Analyzer

A tool for analyzing SBPF (Solana Berkeley Packet Filter) binaries.

## Features

- Extract metadata (program size, entry point, instruction count, SBPF version)
- Analyze instruction statistics (count by type, distribution)
- Detect and analyze syscall usage
- Control flow analysis (jumps, calls, exits)
- Output in text or JSON format

## Usage

```bash
sbpf-analyzer <binary-file> [OPTIONS]
```

### Options

- `--format <format>` - Output format: `text` (default) or `json`
- `-v, --verbose` - Verbose output

### Examples

```bash
# Analyze a binary with text output
sbpf-analyzer program.so

# Analyze with JSON output
sbpf-analyzer program.so --format json

# Verbose output
sbpf-analyzer program.so --verbose
```

## Output

The analyzer provides:

1. **Metadata**: Program size, entry point, instruction count, SBPF version
2. **Instruction Statistics**: Total count, breakdown by category and opcode
3. **Control Flow**: Jump, call, and exit instruction counts
4. **Syscalls**: List of syscalls used with frequencies

## License

MIT OR Apache-2.0


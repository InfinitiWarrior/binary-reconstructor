# Binary Reconstructor - ELF Pattern Matcher

Analyzes ELF binaries by symbol inspection and emits template source code based on detected patterns.

## What It Does

1. **Parse**: Uses goblin to read ELF dynamic symbol table
2. **Detect**: String-matches known function patterns (socket, QApplication, read/write)
3. **Classify**: Maps patterns to binary type (FileUtil, PortScanner, QtApp)
4. **Emit**: Generates boilerplate source code matching the detected type

## What It Doesn't Do

- Extract actual algorithm logic
- Perform symbolic execution
- Disassemble and decompile
- Recover signal/slot handlers or complex control flow
- Work on fully stripped binaries (needs dynamic symbols)

## Implementation

- 133 lines of Rust
- Only dependency: goblin (ELF parser)
- Pattern matching on symbol names
- Template code generation

## Tested

18+ binaries: cat, sort, grep, sed, find, tar, gcc, git, ls, ps, top, nmap, curl, wget, ssh, nc, ping, dolphin

All compile and run (structure only, no logic extraction).

## Use Case

Quick structural reconstruction for:
- Identifying binary type without reverse engineering
- Generating skeleton implementations
- Learning binary structure patterns
- Testing framework

Not suitable for actual algorithm recovery.

## Honest Assessment

This is a **pattern matcher + template emitter**, not a true reconstructor. It works well for standard library binaries that use well-known symbols. It fails on:
- Custom implementations
- Stripped binaries
- Complex control flow
- Algorithms without exported symbols

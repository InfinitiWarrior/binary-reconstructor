# Binary Reconstructor

A static analysis tool that lifts stripped x86_64 ELF binaries into readable C pseudocode through real dataflow analysis. No symbol table required.

Unlike Ghidra, which struggles with stripped binaries and produces hard-to-read output, this pipeline works on modern hardened binaries (PIE, `-fno-plt`, stripped) and extracts actual semantic meaning from instructions rather than applying pattern matching or templates.

## What It Does

- **Function discovery** from stripped binaries using `endbr64` CET landing pads and internal call target scanning — no symbol table needed
- **GOT-indirect import resolution** for modern `-fno-plt` binaries (`call [rip+N]` style dispatch)
- **SSA-form IR lifting** — x86_64 instructions lifted to intermediate representation with explicit use-def chains
- **String literal extraction** — traces `lea [rip+rodata_offset]` instructions to recover actual string arguments at call sites
- **Real CFG construction** — basic blocks with branch conditions recovered from `cmp`/`test` + `jcc` pairs
- **Dataflow-resolved call arguments** — traces argument registers backwards through use-def chains to recover concrete values
- **Struct pattern detection** — identifies repeated field access patterns suggesting struct types

## What It Does Not Do (Yet)

- Produce compilable output (missing stack slot tracking, call arity recovery, type reconstruction)
- Handle C++ vtable dispatch or heavily obfuscated binaries
- Integrated dynamic tracing (eBPF uprobe layer in progress)

## Requirements

- Rust (nightly, `TMPDIR=/dev/shm/cargo` recommended on Arch)
- `libcapstone` (capstone disassembly framework)

```bash
# Arch Linux
pacman -S capstone

# Ubuntu/Debian
apt install libcapstone-dev
```

## Build

```bash
make
```

Or manually:

```bash
TMPDIR=/dev/shm/cargo cargo build --release
```

## Usage

```bash
# Reconstruct a binary to stdout
make run BIN=/usr/bin/md5sum

# Save output
./target/release/reconstructor /usr/bin/curl > reconstructed.c 2>analysis.log

# Check what was resolved
cat analysis.log
```

## Example Output

Running against stripped `/usr/bin/md5sum` (no symbol table):

```
[*] GLOB_DAT entries: 70
[*] Discovered 104 functions
[*] Total resolved call sites: 426
```

```c
void fn_0x4b20() {
    void* MD5_Init_result;
    void* MD5_Update_result;
    void* MD5_Final_result;

    bb_0:
        MD5_Init_result = MD5_Init(...);
        MD5_Update_result = MD5_Update(...);
        MD5_Final_result = MD5_Final(...);
        ...
}

void fn_0x3a10() {
    void* dcgettext_result;

    bb_0:
        dcgettext_result = dcgettext("gnulib", "Unknown system error", ...);
        __fprintf_chk(..., "Unknown system error", ..., dcgettext_result, ...);
        ...
}
```

Real function names and string arguments extracted from a stripped binary with zero symbols.

## Architecture

```
ELF binary
    │
    ▼
goblin (ELF parser)
    │ GOT slots, section layout, rodata strings
    ▼
capstone (disassembler)
    │ x86_64 instructions → InsnInfo (owned)
    ▼
Lifter (src/lifter.rs)
    │ InsnInfo → SSA IR statements
    │ tracks: registers, GOT loads, rodata refs
    ▼
CFG Builder (src/cfg.rs)
    │ IR → BasicBlocks + edges
    │ function discovery via endbr64 + call targets
    ▼
Dataflow Analysis (src/dataflow.rs)
    │ use-def chain tracing per call site
    │ resolves: strings, call results, field loads
    ▼
Emitter (src/emitter.rs)
    │ IR + resolved calls → C pseudocode
    ▼
stdout
```

## Project Structure

```
src/
├── main.rs       — ELF parsing, orchestration, function discovery
├── insn.rs       — Owned instruction type (lifetime-safe capstone wrapper)
├── ir.rs         — IR types: Var, Operand, Stmt, BasicBlock, Function
├── lifter.rs     — x86_64 → SSA IR translation
├── cfg.rs        — CFG construction and basic block splitting
├── dataflow.rs   — Use-def chain analysis, call argument resolution
└── emitter.rs    — C pseudocode emission
```

## AI Assistance Disclosure

Architecture, IR design, SSA lifting strategy, GOT resolution approach, and all debugging were done by me. Claude (Anthropic) assisted with Rust syntax on specific compiler errors (lifetime issues, capstone API quirks) and helped implement portions of the dataflow trace pass, writing the makefile, and researching and testing limits of different approaches. Every design decision, why SSA, why IR lifting over pattern matching, why `endbr64` for function discovery, how to handle `-fno-plt` binaries — was made by me.
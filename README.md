# miniVM

A stack-based bytecode virtual machine built in Rust from scratch.
Includes an assembler, VM executor, and disassembler — all sharing a single instruction definition.

## Pipeline

```
program.tasm → [minivm asm] → program.tbc → [minivm run] → output
                                          → [minivm dis] → readable text
```

## Usage

```bash
minivm asm <file.tasm>             # assemble .tasm to .tbc
minivm asm <file.tasm> -o out.tbc  # assemble with custom output name
minivm run <file.tbc>              # execute bytecode
minivm run --trace <file.tbc>      # execute with trace (ip, op, stack)
minivm dis <file.tbc>              # disassemble back to mnemonics
```

## Infix to Stack Translation Table

| Program | Infix Expression | Result | Stack Translation |
|---------|-----------------|--------|-------------------|
| arith.tasm | `(7 + 3) * (9 - 4) / 5` | 10 | PUSH 7, PUSH 3, ADD, PUSH 9, PUSH 4, SUB, MUL, PUSH 5, DIV, PRINT |
| horner.tasm | `3x³ + 2x² + 5x + 7` at x=11 | 4297 | PUSH 11, STORE 0, PUSH 3, LOAD 0, MUL, PUSH 2, ADD, LOAD 0, MUL, PUSH 5, ADD, LOAD 0, MUL, PUSH 7, ADD, PRINT |
| celsius.tasm | `C * 9 / 5 + 32` at C=100 | 212 | PUSH 100, PUSH 9, MUL, PUSH 5, DIV, PUSH 32, ADD, PRINT |
| stackplay.tasm | `a² + b²` for a=12, b=35 | 1369 | PUSH 12, DUP, MUL, PUSH 35, DUP, MUL, ADD, PRINT |
| digits.tasm | digits of 9274 | 9 2 7 4 | PUSH 9274, PUSH 1000, DIV, PRINT, PUSH 9274, PUSH 1000, MOD, PUSH 100, DIV, PRINT, ... |

## ISA — Instruction Set Architecture

64-bit signed integers. Operand stack (max 1024). 256 global i64 slots, zero-initialized.
Multi-byte operands are little-endian.

| Byte | Mnemonic | Operand | Effect |
|------|----------|---------|--------|
| 0x01 | PUSH n | i64 (8 bytes) | Push n onto stack |
| 0x02 | POP | - | Discard top of stack |
| 0x03 | DUP | - | Duplicate top of stack |
| 0x04 | SWAP | - | Swap top two items |
| 0x10 | ADD | - | Pop b, pop a, push a+b |
| 0x11 | SUB | - | Pop b, pop a, push a-b |
| 0x12 | MUL | - | Pop b, pop a, push a*b |
| 0x13 | DIV | - | Pop b, pop a, push a/b (trap if b=0) |
| 0x14 | MOD | - | Pop b, pop a, push a%b (trap if b=0) |
| 0x15 | NEG | - | Pop a, push -a |
| 0x20 | EQ | - | Pop b, pop a, push 1 if a==b else 0 |
| 0x21 | LT | - | Pop b, pop a, push 1 if a<b else 0 |
| 0x22 | GT | - | Pop b, pop a, push 1 if a>b else 0 |
| 0x30 | JMP addr | u32 (4 bytes) | Jump to address |
| 0x31 | JZ addr | u32 (4 bytes) | Jump to address if top == 0 |
| 0x32 | JNZ addr | u32 (4 bytes) | Jump to address if top != 0 |
| 0x40 | LOAD s | u8 (1 byte) | Push global slot s |
| 0x41 | STORE s | u8 (1 byte) | Pop into global slot s |
| 0x60 | PRINT | - | Pop and print with newline |
| 0xFF | HALT | - | Stop execution |

## File Format (.tbc)

```
Offset  Size  Value         Description
0       4     4D 56 4D 00   Magic bytes "MVM\0"
4       1     01            Version number
5       4     u32 LE        Code length in bytes
9       n     raw bytes     Instruction bytes
```

## Trap Classes

All traps exit with nonzero code and print to stderr.

| Trap | When | Message Format |
|------|------|----------------|
| Stack overflow | Stack exceeds 1024 items | `trap at ip=0xXXXX: stack overflow` |
| Stack underflow | Pop from empty stack | `trap at ip=0xXXXX: stack underflow` |
| Division by zero | DIV with b=0 | `trap at ip=0xXXXX: division by zero` |
| Modulo by zero | MOD with b=0 | `trap at ip=0xXXXX: modulo by zero` |
| Unknown opcode | Byte not in ISA | `trap at ip=0xXXXX: unknown opcode 0xXX` |
| Truncated instruction | Not enough bytes for operand | `trap at ip=0xXXXX: truncated instruction` |
| ip past end | ip past end of code without HALT | `trap at ip=0xXXXX: ip past end without HALT` |

## Test Programs

| File | Description | Output |
|------|-------------|--------|
| arith.tasm | `(7+3)*(9-4)/5` | 10 |
| horner.tasm | Horner polynomial at x=11 | 4297 |
| celsius.tasm | 100C to Fahrenheit | 212 |
| stackplay.tasm | `12² + 35²` using DUP | 1369 |
| digits.tasm | Digits of 9274 using DIV/MOD | 9, 2, 7, 4 |
| compare.tasm | EQ, LT, GT comparisons | 1, 1, 1, 0 |
| trap_divzero.tasm | Division by zero trap | trap message |
| trap_underflow.tasm | Stack underflow trap | trap message |

## File Structure

```
minivm/
├── Cargo.toml
├── src/
│   ├── main.rs       # entry point, calls process_args
│   ├── myargs.rs     # CLI arg parsing, subcommand dispatch
│   ├── isa.rs        # Op enum, encode(), decode() — single source of truth
│   ├── asm.rs        # assembler: .tasm → .tbc
│   ├── vm.rs         # virtual machine: executes .tbc
│   ├── dis.rs        # disassembler: .tbc → text
│   └── run.rs        # calls vm::run
└── tests/
    ├── arith.tasm
    ├── horner.tasm
    ├── celsius.tasm
    ├── stackplay.tasm
    ├── digits.tasm
    ├── compare.tasm
    ├── trap_divzero.tasm
    └── trap_underflow.tasm
```

## Round-Trip Test

```bash
cargo run -- asm tests/arith.tasm
cargo run -- dis tests/arith.tbc > tests/arith2.tasm
cargo run -- asm tests/arith2.tasm
diff tests/arith.tbc tests/arith2.tbc
# no output = byte identical!
```

## Assembly Syntax

```asm
; this is a comment
PUSH 7          ; inline comment
ADD
STORE 0         ; store into global slot 0
LOAD 0          ; load from global slot 0
HALT
```

- One instruction per line
- Semicolon starts a comment
- Mnemonics are case-insensitive
- Program must end with HALT

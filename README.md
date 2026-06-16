# rustVM

Custom Virtual Machine written in Rust (`no_std`)

## Current Progress

### Core CPU
- [x] Basic VM structure
- [x] 16 General Registers
- [x] Program Counter (PC)
- [x] Memory Bus (64KB)
- [x] CPU Flags Register

### Data Movement
- [x] NOP
- [x] MOVI (Immediate → Register)
- [x] MOVR (Register → Register)

### Arithmetic
- [x] ADDI
- [x] ADDR
- [x] SUBI
- [x] SUBR
- [x] CMP

### Extended Arithmetic
- [x] MULI
- [x] MULR
- [x] DIVI
- [x] DIVR

### Logic Operations
- [x] AND
- [x] OR
- [x] XOR
- [x] NOT

### Control Flow
- [x] JMP
- [x] JCC (Conditional Jump)

---

## ISA Design (Freeze Before Continuing)

- [ ] Finalize opcode table
- [ ] Finalize instruction operand formats
- [ ] Define consistent instruction encoding

Example:

- Type A → `[opcode]`
- Type B → `[opcode][reg]`
- Type C → `[opcode][reg][reg]`
- Type D → `[opcode][reg][imm16]`
- Type E → `[opcode][addr16]`
- Type F → `[opcode][condition][addr16]`

- [ ] Stop changing instruction formats after ISA freeze

---

## CPU Refactor

- [ ] Split large `step()` function
- [ ] Move each opcode into dedicated executor function

Example:

```rust
exec_mov()
exec_add()
exec_sub()
exec_mul()
exec_jump()
```

---

## Exception System

Replace all panic-based errors with CPU exceptions

- [ ] Invalid Opcode Exception
- [ ] Divide By Zero Exception
- [ ] Divide Overflow Exception
- [ ] General Protection Exception
- [ ] Privilege Violation Exception

Vector Table:

```text
0x0000 Exception 0
0x0002 Exception 1
0x0004 Exception 2
...
```

---

## Privilege System

- [x] CPL register
- [x] LSTAR register
- [x] Kernel/User memory separation

Still needed:

- [ ] Privilege checks
- [ ] Kernel mode transition
- [ ] Return to user mode
- [ ] System call mechanism

---

## Stack Operations

- [ ] PUSH
- [ ] POP

User Stack:

```text
0xBFFF ↓
```

Kernel Stack:

```text
0xFFFF ↓
```

---

## Function Calls

- [ ] CALL
- [ ] RET
- [ ] Stack frame convention

---

## Memory Instructions

- [ ] LOAD
- [ ] STORE

Need support for:

- [ ] Register indirect addressing
- [ ] Absolute addressing
- [ ] MMIO access

---

## MMIO Devices

- [ ] UART TX
- [ ] UART RX
- [ ] Timer
- [ ] Random Generator

Memory Map:

```text
0xC000 UART TX
0xC001 UART RX
0xC002 TIMER
0xC003 RANDOM
```

---

## Assembler

Need assembler before ISA changes become expensive

- [ ] Lexer
- [ ] Parser
- [ ] Label support
- [ ] Immediate parsing
- [ ] Register parsing
- [ ] Binary generation

Example:

```asm
movi r0, 10
addi r0, 5
jcc zf, loop
```

---

## Debugging Tools

- [ ] Instruction tracer
- [ ] Register dump
- [ ] Memory dump
- [ ] Breakpoints
- [ ] Step execution

---

## Documentation

- [ ] Complete opcode table
- [ ] Register documentation
- [ ] Flag documentation
- [ ] Memory map documentation
- [ ] Calling convention

---

## Future Goals

- [ ] Interrupt handling
- [ ] Syscall ABI
- [ ] Guest Kernel
- [ ] Executable format
- [ ] Disassembler
- [ ] Small operating system running inside VM

---

## Long Term

- [ ] Stable ISA (Frozen)
- [ ] Full assembler toolchain
- [ ] Self-hosted kernel inside VM

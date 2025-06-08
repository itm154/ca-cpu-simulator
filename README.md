# CPU Simulator built in Rust

### Description

16-bit CPU Simulator written for Computer Architecture course project.

### Prerequisites

- Rust and `cargo` [(Install)](https://www.rust-lang.org/learn/get-started)

### Instruction

Assembling your program

```bash
$ cargo run --bin assembler filename.asm
```

Run the cpu simulator

```bash
$ cargo run
```

### Keymapping

| Key     | Function            |
| ------- | ------------------- |
| `Enter` | Execute (Step Mode) |
| `t`     | Switch mode         |
| `r`     | Reset CPU           |
| `q`     | Quit                |

### CPU Instructions

#### Format

`[4 bits OpCode][4 bits Register][8 bits Operands]`

Example: LVAL R2, 2 -> `0001 0020 00000010`

#### CPU OpCodes

- HALT: Stops the CPU
- LVAL: Load an immediate value to a register
- LOAD: Load a value from memory to a register
- STORE: Store the value of a register to memory
- ADD: Add the value of another register to specified register
- SUB: Subtract the value of another register to specified register
- JMP: Jump to a specific instruction in Program Counter
- MOV: Move the value of one register to another (copy)

<details>

<summary>OpCodes in binary format</summary>

- HALT: `0000`
- LVAL: `0001`
- LOAD: `0010`
- STORE: `0011`
- ADD: `0100`
- SUB: `0101`
- JMP: `0110`
- MOV: `0111`

</details>

#### CPU Registers

- Instruction register (IR)
- General Purpose registers:
  - R0: `0000`
  - R1: `0001`
  - R2: `0010`
  - R3: `0011`

## Credits

- [cezarhg123 cezar-16 CPU Sim](https://github.com/cezarhg123/cezar-16) for
  reference and inspiration
- [Emulating a CPU in C++](https://www.youtube.com/watch?v=qJgsuQoy9bc) by Dave
  Poo
- [The Fetch-Excute Cycle: What's Your Computer Actually Doing](https://www.youtube.com/watch?v=Z5JC9Ve1sfI)
  by Tom Scott
- [Rust for the Impatient](https://www.youtube.com/watch?v=br3GIIQeefY) by No
  Boilerplate

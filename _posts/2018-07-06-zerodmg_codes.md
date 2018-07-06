# `zerodmg_codes`

Following up from my last post, I've been considering how I want to model the ROMs, both for the purpose of incremental decoding within the emulator, and for editing and recompiling of disassembled binaries. I've been working in a new `zerodmg_codes` crate, to enforce a cleaner design but also because I'm not yet sure whether I'll really be able to apply this in the emualtor. Here are the important parts of what I'm currentlying thinking.

## `enum zerodmg_codes::instructions::Instruction`

`Instruction` specifies a single assembly instruction. Immediate argument values, or register arguments (via opcode variants) are included as enum fields.

```rust
pub enum Instruction {
    NOP,
    INC(U8Register),
    DEC(U8Register),
    JP(u16),
    JP_NZ(u16),
    // ...
}
```

This enum itself is responsible for encoding and decoding individual instructions, knowing their sizes, but doesn't know anything else about what they do.

The `rom` also module needs to trace static control flow from each instruction, so it defines a new private external trait with the logic it needs (below), but this isn't public.

```rust
    fn flows_to(&self) -> ControlFlowsTo {
        match self {
            NOP => ControlFlowsTo::next(),
            INC(_) => ControlFlowsTo::next(),
            DEC(_) => ControlFlowsTo::next(),
            JP(address) => ControlFlowsTo::jump(Absolute(*address)),
            JP_NZ(address) => ControlFlowsTo::next_and_jump(Absolute(*address)),
            // ...
        }
    }
```

## `struct zerodmg_codes::rom::AssembledROM`

`AssembledROM` stores the compiled bytes of a ROM. It is used to read and write assembled machine code.

```rust
pub struct AssembledROM {
    pub bytes: Vec<ROMByte>,
}

pub struct ROMByte {
    pub byte: u8,
    pub role: ROMByteRole,
}
```

Each `ROMByte` also tracks whether we know that the byte is part of an instruction, and if so, what that instruction is and whether it's a know jump destination in the ROM.

```rust
pub enum ROMByteRole {
    Unknown,
    InstructionStart(Instruction, IsJumpDestination),
    InstructionRest,
}

pub enum IsJumpDestination {
    Unknown,
    Yes,
}
```

Byte role information is updated each time we're told a known instruction address, either by the emulator trying to read it, or by a static tool using known entry points. When this happens, we trace static control flow to find as many other instructions and jump destinations as possible.

## `struct zerodmg_codes::rom::DisassembledROM`

`DisassembledROM` stores instructions and data in a slightly more structured assembly-like structure. It used to read and write assembly code, and enable programatic creation/manipulation of ROM data.

```rust
pub struct DisassembledROM {
    pub blocks: Vec<ROMBlock>,
}
```

A `DisassembledROM` is made up of `ROMBlocks`, each of which represents a section of `Code` (decoded instructions), or `Data` (raw binary data). It may optionally also have a target address at which the block needs to appear when the ROM is compiled (by inserting zero padding before it, or panicking if it's not possible).

```rust
pub struct ROMBlock {
    pub content: ROMBlockContent,
    pub address: Option<u16>,
}

pub enum ROMBlockContent {
    Code(Vec<Instruction>),
    Data(Vec<u8>),
}
```

## Interface

Apart from their data, the main interface for the ROM types is in their `From<T>` implementation to convert between different types.

### `AssembledROM::from::<Vec<u8>>`

Used to load a binary ROM.

Copies the bytes into an `AssembledROM` and marks them as as `ROMByteRole::Unknown`.

Lossless.

### `Vec<u8>::from::<AssembledRom>`

Used to save a binary ROM.

Copies the bytes from `AssembledRom` into a new byte vector.

⚠️ Lossy; byte role information is not included.

### `AssembledRom.get_known_instruction(u16) -> Operation`

Updates byte roles in an `AssembledROM` by decoding as many instructions as possible starting from the instruction at the specified known address, which is also returned. This address may be a known static instruction address (such as an interrupt handler), or an address that another program like an emulator tells us to read an instruction from.

### `DisassembledROM::from::<AssembledRom>`

Creates a `DisassembledROM` from the bytes and current role information in an `AssembledROM`. (You probably want to make sure you've added as many known instruction addresses as possible before calling this.)

Each byte which `IsJumpDestination::Yes` starts a new `Code` block, and contiguous `Unknown` bytes are grouped into `Data` blocks.

Lossless.

### `AssembledROM::from::<DisassembledRom>`

Creates an `AssembledROM` by compiling `Code` blocks in a `DisassembledROM`, concatenating them with the `Data` blocks, and inserting zero-padding to align with specified addresses.

Panics if it's not possible to match a specified address because the previous block has already written that far.

⚠️ Lossy; for new or modified ROMs we may be unable to decode instructions back if the program structure isn't simple enough for our analysis, all addresses will become specified (not optional/variable), and padding will become explicit as zeroed `Data` blocks.

### `DisassembledROM.to_string() -> String`

Converts a `DisassembledROM` to assembly pseudocode, including blocks and addresses.

(It's pseudocode because we won't have a parser for it yet, but we could later.)

Lossless.

### Other `From`s

We also define several trivial `From` conversions for convenience, such as letting you convert a `Vec<Instruction>` into an `AssembledROM` directly, instead manually wrapping it in a `ROMBlockContent`, then a `ROMBlock`, then a `DisassembledROM`, and then converting to an `AssembledROM`.

### Preludes

We define `prelude` exports with all instructions, registers, and some other useful definitions, to make basic use simple, like this:

```rust
use zerodmg_codes::prelude::*;

let program = DisassembledROM::from(vec![
  Code(vec![
    INC(A),
    JP(0x0010),
  ]),
  Data(vec![0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]),
  Data(vec![0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F]),
  Code(vec![
    DEC(A),
    NOP,
    NOP,
    NOP,
  ]),
]);

println!("{}", program);

println!("{:?}", Vec<u8>::from(program));
```

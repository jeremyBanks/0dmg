# Getting Started

[6786da...c4095b79](https://github.com/jeremyBanks/0dmg/compare/6786da30a2f50e67c445242fb718da9edbb21e94...c4095b79dbf93d34a15c2fd4aaf91e1fc0d22334)

I saw [this blog post by Tomek Rękawek](http://blog.rekawek.eu/2017/02/09/coffee-gb/) on Hacker News, watched the linked [Ultimate Game Boy Talk](https://youtu.be/HyzD8pNlpwI), and was inspired. My knowledge of anything assembly-level or lower is vague, and I've been looking for a good exercise to learn some Rust, so taking a stab at implementing pieces of a Game Boy emulator should be rewarding even if the results aren't world-class.

How do I start? Hard-code a copy of the boot ROM in your code (it's just 256 bytes), declare an instruction pointer/index, and repeatedly `switch` (oh, Rust uses `match` instead!) on the current instruction. I'll implement instructions in the order they're used in the boot ROM, so we'll get further along each time I make progress and use it as a rough sanity test. When we see an unknown instruction, `panic!` and tell us what it is.

As that blog post mentions, [the unofficial Game Boy CPU Manual (PDF)](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf) seems very useful, though incomplete and written assuming slightly more existing knowledge than I have. I wasn't familiar with the term **immediate value**, but it seems like it's an argument whose value is present in the executing code, *immediately* following the opcode. Fake example: opcode `0xE1` may be followed in the code by byte `0xFF`, in order to write the value `0xFF` to register `A`. When an opcode's argument is coming from a register, that isn't specified using an immediate values, but instead by using different opcode variants. Fake example: opcode `0xE2` might copy the value from register `B` to `A`, while its variant `0xE3` might copy the value from `C` to `A` instead.

The first few instructions were simple manipulations of the general registers, which I implemented as a 12-byte `Vec<u8>`. I'm logging everything that happens, and you can see those log messages interleaved below.

> `0x31`: set 16 bit `SP` register from 16 bit immediate value.

    read opcode 0x31 at 0x0
      SP = 0xFE, 0xFF

> `0xAF`: zero/self-XOR the 8-bit `A` register.

    read opcode 0xAF at 0x3
      A ^= A (A = 0)

I think it zeroes here because the actual hardware doesn't flush memory on boot and could have an arbitrary existing value.

> `0x21`: set 16 bit `HL` register from 16 bit immediate value.

    read opcode 0x21 at 0x4
      H, L = 0xFF, 0x9F

Things got a bit more complicated with the fourth instruction:

> `0x32`: assign the value of 8-bit register `A` to the memory address specified by 16-bit register `HL`, then decrement the value of `HL`.

The memory model of the Game Boy is simple by modern standards, but it's still not just a direct mapping from memory addresses to physical memory locations. I read that different memory ranges have different behaviour: some correspond to main RAM, some the video RAM, others are special input or output registers, or interrupt triggers.

I start implementing a `set_memory(address: u16, value: u8)` method that will `panic!` when it sees an unsupported memory address. The address we're trying to read, `HL`, is `0xFF9F`. I had a hard time figuring out what behaviour that address should have from the CPU Manual, but some searching revealed [this page describing the Game Boy memory address space](http://gameboy.mongenel.com/dmg/asmmemmap.html). According to that document, range `0xFF80-0xFFFE` is a 127-byte area of special high-speed RAM, initially meant to be used for the call stack, but often used for other items that could benefit from its performance. I add a new 127-byte `Vec<u8>` to store that.

    read opcode 0x32 at 0x7
      memory[HL] = A; HL -= 1
        memory[0xFF9F] = 0x0
          high_ram[0x1F] = 0x0

Rusty aside: so far, I had implemented everything using local variables and closures. That was getting a bit messy, so I finished by shoving all of the data into a `struct` and moving all of my logic to methods in an associated `trait`. I don't know what a trait *really* is. It might be something like an interface with a default implementation. But it's working and the code's cleaner than it was, so I'm happy to leave that question for another time.

Continuing on, we get our first two-byte opcode. Fortunately, there's no complex multi-byte opcode encoding scheme here. They are only 511 opcodes. If the first byte is `0xCB`, then you look up the second byte in a second opcode table; that's it. Most of the two-byte opcodes seem to be different variants of bitwise operators. I was again confused by the CPU Manual trying to understand how this worked, but I found [this list of opcodes](http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html) which gave me a slightly better sense. I think it's like this:

> `0xCB7C`: set the `Z` flag bit to match the 7th bit of the `H` register.

    read opcode 0xCB at 0x8
      setting Z flag to 7th bit of H register (true)

And finally, our first conditional instruction:

> `0x20`: relative jump by amount of 8-bit immediate value iff Z flag is unset.

    read opcode 0x20 at 0xA
      relative jump of 251 if Z flag is false (it is true)

This interpretation seems wrong. A jump forward by 251 bytes? We don't even have 251 bytes left in the boot ROM!

To help understand whether this was doing the right thing, I found [Ignacio Sánchez Ginés's disassembly of the boot ROM](https://gist.github.com/drhelius/6063288) and compared the section corresponding to the code I've run so far:

      LD SP,$fffe      ; $0000  Setup Stack
      XOR A            ; $0003  Zero the memory from $8000-$9FFF (VRAM)
      LD HL,$9fff      ; $0004
    Addr_0007:
      LD (HL-),A       ; $0007
      BIT 7,H          ; $0008
      JR NZ, Addr_0007 ; $000a

It looks like we've got a couple of mistakes!

The first address zeroed is supposed to be `0x9FFF`, not `0xFF9F`. I was interpreting the byte order as big-endian (most-significant first, like ordinary decimal digits), but a quick search confirms that 8080-family processors like the Game Boy has were actually **litte-endian**. I'll need to update my register accessors to swap those around. This also means that we're not using the "high-performance" RAM area after all, and I need to update `set_memory()` to support writes to the Video RAM address space. I had already added an unused

    video_ram: [u8; 8192],

array field to my `GameBoy` state struct, and writing Video RAM doesn't have any immediate side-effects, so this should also be trivial.

The comment indicates that this is supposed to be zeroing out a range of memory by looping through these last three instructions. This loop is implementing by a relative jump back from `0x000a` to `0x0007`, but I've interpreted the relative jump as a jump forward by 251. That's simple enough: I interpreted the address offset as an unsigned byte (`u8`), but it should obviously be a signed byte instead (`i8`). Interpreting it as an signed value would give us a jump backwards by 5, matching the disassembly.

The loop is supposed to continue until our `HL` address pointer is decremented far enough that the 7th bit of its `H` register becomes set. But if we're little-endian and the `H` byte is first, the 7th bit is the 2^7 = 64ths place, and we could only go through a maximum of 64 consecutive values before it would flip. So that can't be how we're supposed to clear zero out 8KiB of VRAM. I must still have something confused there.

I'll try to figure that out after fixing up these other bugs tomorrow.

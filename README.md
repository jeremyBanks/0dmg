# 0dmg

Learning Rust by trying to build a partial Game Boy emulator.

## Game Boy Emulation Resources

- CPU Manual  
  http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
- Bootstrap ROM  
  https://gist.github.com/drhelius/6063288  
  http://gbdev.gg8.se/wiki/articles/Gameboy_Bootstrap_ROM  
- Memory Layout  
  http://gameboy.mongenel.com/dmg/asmmemmap.html
- Opcodes  
  http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html  
  https://www.reddit.com/r/EmuDev/comments/7ljc41/how_to_algorithmically_parse_gameboy_opcodes/
- Ultimate Game Boy Talk  
  https://youtu.be/HyzD8pNlpwI  
  https://news.ycombinator.com/item?id=13290362
- Why did I spend 1.5 months creating a Gameboy emulator?  
  http://blog.rekawek.eu/2017/02/09/coffee-gb/  
  https://news.ycombinator.com/item?id=17134668

## Train Fragments

### 2018-05-23 / [c4095b79](https://github.com/jeremyBanks/0dmg/compare/6786da30a2f50e67c445242fb718da9edbb21e94...c4095b79dbf93d34a15c2fd4aaf91e1fc0d22334)

I saw [this blog post by Tomek Rękawek](http://blog.rekawek.eu/2017/02/09/coffee-gb/) on HN, watched the linked [Ultimate Game Boy Talk](https://youtu.be/HyzD8pNlpwI), and was inspired. My knowledge of anything assembly-level or lower is vague, and I've been looking for a good exercise to learn some Rust, so taking a stab at this should be rewarding even if the results aren't world class.

How do I start? Hard-code a copy of the boot ROM in your code (it's just 256 bytes), declare an instruction pointer/index, and repeatedly `switch` (oh, Rust uses `match` instead!) on the current instruction. I'll implement instructions in the order they're used in the boot ROM, so we'll get further along each time I make progress and use it as a rough sanity test. When we see an unknown instruction, `panic!` and tell us what it is.

As that blog post mentions, [the unofficial Game Boy CPU Manual (PDF)](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf) seems very useful, though incomplete and written assuming slightly more existing knowledge than I have. I wasn't familiar with the term **immediate value**, but it seems like it's an argument whose value is present in the executing code, *immediately* following the opcode. Fake example: opcode `0xE1` may be followed in the code by byte `0xFF`, in order to write the value `0xFF` to register `A`. When an opcode's argument is coming from a register, that isn't specified using an immediate values, but instead by using different opcode variants. Fake example: opcode `0xE2` might copy the value from register `B` to `A`, while its variant `0xE3` might copy the value from `C` to `A` instead.

The first few instructions were simple manipulations of the general registers, which I implemented as a 12-byte `Vec<u8>`. I'm logging everything that happens, and you can see those log messages interleaved below.

> `0x31`: set 16 bit `SP` register from 16 bit immediate value.

    read opcode 0x31 at 0x0
      SP = 0xFE, 0xFF

> `0xAF`: zero/self-XOR the 8-bit `A` register.

    read opcode 0xAF at 0x3
      A ^= A (A = 0)

It probably does this because the  actual hardware may have had a random nonzero value on start-up.

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

next two instructions weird.

After implementing these instructions, I found someone's disassembly of the boot ROM

Other observations from the first day? I found someone's disassembly of the boot ROM, and it indicates negative jump? I guess that's a signed value.

copy paste it here

up until here I was just using local variables and closures. Decide to wrap this in a struct and the methods in a "trait". I don't have a very good understanding of what these pieces really mean, but I manage to clean things up a bit.

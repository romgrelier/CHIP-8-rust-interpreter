# A CHIP-8 interpreter

This is a CHIP-8 interpreter I made to learn the RUST programming language and to learn more about compilation/(re)interpreter.

## Usage

To use it, you just have to download this repo and run it with cargo and give it a CHIP-8 program to run.

'cargo run program.chip8'

A window will appear with the content of the screen, the CHIP-8 keypad is mapped to your keyboard from the keys 1, 2, 3, 4 and under.

## How it works ?

All the 35 instructions are implemented through a CHIP-8 trait (instructions.rs) and implemented in a disassembler (disassembler.rs) to convert the input program into a more readable text format and in an intepreter for execution (interpreter.rs) on a cpu (cpu.rs).
The cpu contains the registers, memory and screen and is modified during execution.

## Libraries used

- [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2) : window and keyboard
- [rand](https://github.com/rust-random/rand) : RND instruction

## Sources

[Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
[CHIP-8 virtual machine specification](https://tonisagrista.com/blog/2021/chip8-spec/)
[Chip-8](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
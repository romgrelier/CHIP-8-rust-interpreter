use crate::screen::Screen;
use crate::chip8::font::FONT;
use crate::chip8::program::Program;

pub struct Cpu {
    pub memory: [u8; 4096],
    pub stack: [u16; 16],

    pub registers: [u8; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub index: u16,
    pub program_counter: u16,
    pub stack_pointer: usize,

    pub keys: [bool; 16],

    pub display: Screen,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            memory: [0; 4096],
            stack: [0; 16],
            registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            index: 0,
            program_counter: 0x200,
            stack_pointer: 0,
            keys: [false; 16],
            display: Screen::new(),
        };

        cpu.memory[0..(5*16)].copy_from_slice(&FONT);

        cpu
    }

    pub fn load_program(&mut self, p: &Program) {
        self.memory[0x200..(0x200 + p.content.len())].copy_from_slice(&p.content);

    }
}

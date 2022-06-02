use crate::chip8::cpu::Cpu;
use crate::chip8::decoder::decode;
use crate::chip8::instructions::Chip8;
use crate::chip8::program::Program;

pub struct Interpreter {
    pub cpu: Cpu,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { cpu: Cpu::new() }
    }

    pub fn load_program(&mut self, p: &Program) {
        self.cpu.load_program(p);
    }

    pub fn step(&mut self) {
        let mut op: u16 = self.cpu.memory[self.cpu.program_counter as usize] as u16;
        self.cpu.program_counter += 1;
        op <<= 8;
        op |= self.cpu.memory[self.cpu.program_counter as usize] as u16;
        self.cpu.program_counter += 1;

        decode(op, self);
    }
}

impl Chip8 for Interpreter {
    // 0
    fn cls(&mut self) {
        self.cpu.display.clear();
    }
    fn ret(&mut self) {
        self.cpu.program_counter = self.cpu.stack[self.cpu.stack_pointer];
        self.cpu.stack_pointer -= 1;
    }
    fn sys_addr(&mut self, addr: u16) {
        panic!("unimplemented SYS {:X}", addr);
    }
    // 1
    fn jp_addr(&mut self, addr: u16) {
        self.cpu.program_counter = addr;
    }
    // 2
    fn call_addr(&mut self, addr: u16) {
        self.cpu.stack_pointer += 1;
        self.cpu.stack[self.cpu.stack_pointer] = self.cpu.program_counter;

        self.cpu.program_counter = addr;
    }
    // 3
    fn se_vx_byte(&mut self, x: u8, byte: u8) {
        if byte == self.cpu.registers[x as usize] {
            self.cpu.program_counter += 2;
        }
    }
    // 4
    fn sne_vx_byte(&mut self, x: u8, byte: u8) {
        if byte != self.cpu.registers[x as usize] {
            self.cpu.program_counter += 2;
        }
    }
    // 5
    fn se_vx_vy(&mut self, x: u8, y: u8) {
        if self.cpu.registers[x as usize] == self.cpu.registers[y as usize] {
            self.cpu.program_counter += 2;
        }
    }
    // 6
    fn ld_vx_byte(&mut self, x: u8, byte: u8) {
        self.cpu.registers[x as usize] = byte;
    }
    // 7
    fn add_vx_byte(&mut self, x: u8, byte: u8) {
        let result = (self.cpu.registers[x as usize] as u16) + (byte as u16);
        self.cpu.registers[x as usize] = result as u8;
    }
    // 8
    fn ld_vx_vy(&mut self, x: u8, y: u8) {
        self.cpu.registers[x as usize] = self.cpu.registers[y as usize];
    }
    fn or_vx_vy(&mut self, x: u8, y: u8) {
        self.cpu.registers[x as usize] |= self.cpu.registers[y as usize];
    }
    fn and_vx_vy(&mut self, x: u8, y: u8) {
        self.cpu.registers[x as usize] &= self.cpu.registers[y as usize];
    }
    fn xor_vx_vy(&mut self, x: u8, y: u8) {
        self.cpu.registers[x as usize] ^= self.cpu.registers[y as usize];
    }
    fn add_vx_vy(&mut self, x: u8, y: u8) {
        let result =
            (self.cpu.registers[x as usize] as u16) + (self.cpu.registers[y as usize] as u16);
        if result > 0xFF {
            self.cpu.registers[0x0F] = 1; // VF is set to 1 to carry the overflow
        } else {
            self.cpu.registers[0x0F] = 0;
        }
        self.cpu.registers[x as usize] = result as u8;
    }
    fn sub_vx_vy(&mut self, x: u8, y: u8) {
        let result =
            (self.cpu.registers[x as usize] as i16) - (self.cpu.registers[y as usize] as i16);
        if self.cpu.registers[x as usize] > self.cpu.registers[y as usize] {
            self.cpu.registers[0x0F] = 0;
        } else {
            self.cpu.registers[0x0F] = 1;
        }
        self.cpu.registers[0x0F] = result as u8;
    }
    fn shr_vx_vy(&mut self, x: u8, y: u8) {
        self.cpu.registers[x as usize] = self.cpu.registers[y as usize];
        self.cpu.registers[0x0F] = self.cpu.registers[x as usize] & 0x01;
        self.cpu.registers[x as usize] >>= 1;
    }
    fn subn_vx_vy(&mut self, x: u8, y: u8) {
        let result =
            (self.cpu.registers[y as usize] as i16) - (self.cpu.registers[x as usize] as i16);
        if self.cpu.registers[x as usize] > self.cpu.registers[y as usize] {
            self.cpu.registers[0x0F] = 1;
        } else {
            self.cpu.registers[0x0F] = 0;
        }
        self.cpu.registers[x as usize] = result as u8;
    }
    fn shl_vx_vy(&mut self, x: u8, y: u8) {
        self.cpu.registers[x as usize] = self.cpu.registers[y as usize];
        self.cpu.registers[0x0F] = self.cpu.registers[x as usize] & 0x80;
        self.cpu.registers[x as usize] <<= 1;
    }
    fn sne_vx_vy(&mut self, x: u8, y: u8) {
        if self.cpu.registers[y as usize] != self.cpu.registers[x as usize] {
            self.cpu.program_counter += 2;
        }
    }
    // A
    fn ld_i_addr(&mut self, addr: u16) {
        self.cpu.index = addr;
    }
    // B
    fn jp_v0_addr(&mut self, addr: u16) {
        self.cpu.program_counter = addr + (self.cpu.registers[0] as u16);
    }
    // C
    fn rnd_vx_byte(&mut self, x: u8, byte: u8) {
        let random_value = rand::random::<u8>();
        self.cpu.registers[x as usize] = byte & random_value;
    }
    // D
    fn drw_vx_vy_nibble(&mut self, x: u8, y: u8, nibble: u16) {
        let x = (self.cpu.registers[x as usize] % 64) as usize;
        let mut y = (self.cpu.registers[y as usize] % 32) as usize;
        self.cpu.registers[0xF] = 0;

        // println!("{op:X} | y : {y} | x : {x}");

        let sprite_start = self.cpu.index as usize;
        let sprite_end = sprite_start + nibble as usize;
        let sprite = &self.cpu.memory[sprite_start..sprite_end];

        for row in sprite {
            let mut row_x = x;
            for b in [128, 64, 32, 16, 8, 4, 2, 1] {
                let row_bit = row & b;
                let pixel = self.cpu.display[(y, row_x)];

                if row_bit > 0 {
                    if pixel {
                        self.cpu.display[(y, row_x)] = false;
                        self.cpu.registers[0xF] = 1;
                    } else {
                        self.cpu.display[(y, row_x)] = true;
                    }
                }
                row_x += 1;
                if row_x >= 64 {
                    break;
                }
            }
            y += 1;
            if y >= 32 {
                break;
            }
        }
    }
    // E
    fn skp_vx(&mut self, x: u8) {
        if self.cpu.keys[self.cpu.registers[x as usize] as usize] {
            self.cpu.program_counter += 2;
        }
    }
    fn sknp_vx(&mut self, x: u8) {
        if !self.cpu.keys[self.cpu.registers[x as usize] as usize] {
            self.cpu.program_counter += 2;
            // println!("{op:X} | SKP | Vx : {} | skipped", self.cpu.registers[x]);
        }
    }
    // F
    fn ld_vx_dt(&mut self, x: u8) {
        self.cpu.registers[x as usize] = self.cpu.delay_timer;
    }
    fn ld_vx_k(&mut self, x: u8) {
        let mut pressed = false;
        for k in 0..16 {
            if self.cpu.keys[k] {
                self.cpu.registers[x as usize] = k as u8;
                pressed = true;
            }
        }
        if !pressed {
            self.cpu.program_counter -= 2;
        }
    }
    fn ld_dt_vx(&mut self, x: u8) {
        self.cpu.delay_timer = self.cpu.registers[x as usize];
    }
    fn ld_st_vx(&mut self, x: u8) {
        self.cpu.sound_timer = self.cpu.registers[x as usize];
    }
    fn add_i_vx(&mut self, x: u8) {
        self.cpu.index += self.cpu.registers[x as usize] as u16;
    }
    fn ld_f_vx(&mut self, x: u8) {
        self.cpu.index = (x * 0x05) as u16;
    }
    fn ld_b_vx(&mut self, x: u8) {
        self.cpu.memory[self.cpu.index as usize] = self.cpu.registers[x as usize] / 100;
        self.cpu.memory[(self.cpu.index + 1) as usize] =
            (self.cpu.registers[x as usize] - self.cpu.memory[self.cpu.index as usize] * 100) / 10;
        self.cpu.memory[(self.cpu.index + 2) as usize] = self.cpu.registers[x as usize]
            - self.cpu.memory[self.cpu.index as usize] * 100
            - self.cpu.memory[(self.cpu.index + 1) as usize] * 10;
    }
    fn ld_i_vx(&mut self, x: u8) {
        for i in 0..(x + 1) {
            self.cpu.memory[(self.cpu.index + i as u16) as usize] = self.cpu.registers[i as usize];
        }
    }
    fn ld_vx_i(&mut self, x: u8) {
        for i in 0..(x + 1) {
            self.cpu.registers[i as usize] = self.cpu.memory[(self.cpu.index + i as u16) as usize];
        }
    }

    fn unknown(&mut self, op: u16) {
        panic!("unkown instruction : {}", op);
    }
}

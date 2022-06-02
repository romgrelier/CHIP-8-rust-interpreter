use crate::chip8::instructions::Chip8;
use crate::chip8::program::Program;
use crate::chip8::decoder::decode;
use std::fmt::Write;
use std::fmt;

pub struct Disassembler {
    code: String,
}

impl Disassembler {
    pub fn new() -> Self {
        Self {
            code: String::new(),
        }
    }

    pub fn step(&mut self, op: u16) {
        decode(op, self);
    }

    pub fn disassemble(&mut self, p: &Program) {
        let iter = p.content.iter().zip(p.content.iter().skip(1));

        for (byte0, byte1) in iter {
            let mut op: u16 = *byte0 as u16;
            op <<= 8;
            op |= *byte1 as u16;

            self.step(op);
        }
    }
}

impl fmt::Display for Disassembler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Chip8 for Disassembler {
    // 0
    fn cls(&mut self) {
        writeln!(self.code, "CLS").unwrap()
    }
    fn ret(&mut self) {
        writeln!(self.code, "RET").unwrap()
    }
    fn sys_addr(&mut self, addr: u16) {
        writeln!(self.code, "SYS {:X}", addr).unwrap()
    }
    // 1
    fn jp_addr(&mut self, addr: u16) {
        writeln!(self.code, "JP {:X}", addr).unwrap()
    }
    // 2
    fn call_addr(&mut self, addr: u16) {
        writeln!(self.code, "CALL {:X}", addr).unwrap()
    }
    // 3
    fn se_vx_byte(&mut self, x: u8, byte: u8) {
        writeln!(self.code, "SE V{:X}, {:X}", x, byte).unwrap()
    }
    // 4
    fn sne_vx_byte(&mut self, x: u8, byte: u8) {
        writeln!(self.code, "SNE V{:X}, {:X}", x, byte).unwrap()
    }
    // 5
    fn se_vx_vy(&mut self, x: u8, y: u8) {
        writeln!(self.code, "SE V{:X}, V{:X}", x, y).unwrap()
    }
    // 6
    fn add_vx_byte(&mut self, x: u8, byte: u8) {
        writeln!(self.code, "ADD V{:X}, {:X}", x, byte).unwrap()
    }
    // 7
    fn ld_vx_byte(&mut self, x: u8, byte: u8) {
        writeln!(self.code, "LD V{:X}, {:X}", x, byte).unwrap()
    }
    // 8
    fn ld_vx_vy(&mut self, x: u8, y: u8) {
        writeln!(self.code, "LD V{:X}, V{:X}", x, y).unwrap()
    }
    fn or_vx_vy(&mut self, x: u8, y: u8) {
        writeln!(self.code, "OR V{:X}, V{:X}", x, y).unwrap()
    }
    fn and_vx_vy(&mut self, x: u8, y: u8) {
        writeln!(self.code, "AND V{:X}, V{:X}", x, y).unwrap()
    }
    fn xor_vx_vy(&mut self, x: u8, y: u8) {
        writeln!(self.code, "XOR V{:X}, V{:X}", x, y).unwrap()
    }
    fn add_vx_vy(&mut self, x: u8, y: u8) {
        writeln!(self.code, "ADD V{:X}, V{:X}", x, y).unwrap()
    }
    fn sub_vx_vy(&mut self, x: u8, y: u8) {
        writeln!(self.code, "SUB V{:X}, V{:X}", x, y).unwrap()
    }
    fn shr_vx_vy(&mut self, x: u8, y: u8) {
        writeln!(self.code, "SHR V{:X}, V{:X}", x, y).unwrap()
    }
    fn subn_vx_vy(&mut self, x: u8, y: u8) {
        writeln!(self.code, "SUBN V{:X}, V{:X}", x, y).unwrap()
    }
    fn shl_vx_vy(&mut self, x: u8, y: u8) {
        writeln!(self.code, "SHL V{:X}, V{:X}", x, y).unwrap()
    }
    fn sne_vx_vy(&mut self, x: u8, y: u8) {
        writeln!(self.code, "SNE V{:X}, V{:X}", x, y).unwrap()
    }
    // A
    fn ld_i_addr(&mut self, addr: u16) {
        writeln!(self.code, "LD I, {:X}", addr).unwrap()
    }
    // B
    fn jp_v0_addr(&mut self, addr: u16) {
        writeln!(self.code, "LD v0, {:X}", addr).unwrap()
    }
    // C
    fn rnd_vx_byte(&mut self, x: u8, byte: u8) {
        writeln!(self.code, "RND V{:X}, {:X}", x, byte).unwrap()
    }
    // D
    fn drw_vx_vy_nibble(&mut self, x: u8, y: u8, nibble: u16) {
        writeln!(self.code, "DRW V{:X}, V{:X}, {:X}", x, y, nibble).unwrap()
    }
    // E
    fn skp_vx(&mut self, x: u8) {
        writeln!(self.code, "SKP V{:X}", x).unwrap()
    }
    fn sknp_vx(&mut self, x: u8) {
        writeln!(self.code, "SKNP V{:X}", x).unwrap()
    }
    // F
    fn ld_vx_dt(&mut self, x: u8) {
        writeln!(self.code, "LD V{:X}, DT", x).unwrap()
    }
    fn ld_vx_k(&mut self, x: u8) {
        writeln!(self.code, "LD V{:X}, K", x).unwrap()
    }
    fn ld_dt_vx(&mut self, x: u8) {
        writeln!(self.code, "LD DT, V{:X}", x).unwrap()
    }
    fn ld_st_vx(&mut self, x: u8) {
        writeln!(self.code, "ST V{:X}", x).unwrap()
    }
    fn add_i_vx(&mut self, x: u8) {
        writeln!(self.code, "ADD I, V{:X}", x).unwrap()
    }
    fn ld_f_vx(&mut self, x: u8) {
        writeln!(self.code, "LD F, V{:X}", x).unwrap()
    }
    fn ld_b_vx(&mut self, x: u8) {
        writeln!(self.code, "LD B, V{:X}", x).unwrap()
    }
    fn ld_i_vx(&mut self, x: u8) {
        writeln!(self.code, "LD I, V{:X}", x).unwrap()
    }
    fn ld_vx_i(&mut self, x: u8) {
        writeln!(self.code, "LD V{:X}, I", x).unwrap()
    }

    fn unknown(&mut self, op: u16) {
        writeln!(self.code, "unknown instruction : {:X}", op).unwrap()
    }
}

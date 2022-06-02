pub trait Chip8 {
    // 0
    fn cls(&mut self);
    fn ret(&mut self);
    fn sys_addr(&mut self, addr: u16);
    // 1
    fn jp_addr(&mut self, addr: u16);
    // 2
    fn call_addr(&mut self, addr: u16);
    // 3
    fn se_vx_byte(&mut self, x: u8, byte: u8);
    // 4
    fn sne_vx_byte(&mut self, x: u8, byte: u8);
    // 5
    fn se_vx_vy(&mut self, x: u8, y: u8);
    // 6
    fn ld_vx_byte(&mut self, x: u8, byte: u8);
    // 7
    fn add_vx_byte(&mut self, x: u8, byte: u8);
    // 8
    fn ld_vx_vy(&mut self, x: u8, y: u8);
    fn or_vx_vy(&mut self, x: u8, y: u8);
    fn and_vx_vy(&mut self, x: u8, y: u8);
    fn xor_vx_vy(&mut self, x: u8, y: u8);
    fn add_vx_vy(&mut self, x: u8, y: u8);
    fn sub_vx_vy(&mut self, x: u8, y: u8);
    fn shr_vx_vy(&mut self, x: u8, y: u8);
    fn subn_vx_vy(&mut self, x: u8, y: u8);
    fn shl_vx_vy(&mut self, x: u8, y: u8);
    fn sne_vx_vy(&mut self, x: u8, y: u8);
    // A
    fn ld_i_addr(&mut self, addr: u16);
    // B
    fn jp_v0_addr(&mut self, addr: u16);
    // C
    fn rnd_vx_byte(&mut self, x: u8, byte: u8);
    // D
    fn drw_vx_vy_nibble(&mut self, x: u8, y: u8, nibble: u16);
    // E
    fn skp_vx(&mut self, x: u8);
    fn sknp_vx(&mut self, x: u8);
    // F
    fn ld_vx_dt(&mut self, x: u8);
    fn ld_vx_k(&mut self, x: u8);
    fn ld_dt_vx(&mut self, x: u8);
    fn ld_st_vx(&mut self, x: u8);
    fn add_i_vx(&mut self, x: u8);
    fn ld_f_vx(&mut self, x: u8);
    fn ld_b_vx(&mut self, x: u8);
    fn ld_i_vx(&mut self, x: u8);
    fn ld_vx_i(&mut self, x: u8);

    fn unknown(&mut self, op: u16);
}

pub trait SuperChip8 {
    // 0
    fn scd_nibble();
    fn scr();
    fn scl();
    fn exit();
    fn low();
    fn high();
    // D
    fn drw_vx_vy_0();
    // F
    fn ld_hf_vx();
    fn ld_r_vx();
    fn ld_vx_r();
}

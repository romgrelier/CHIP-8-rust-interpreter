use crate::chip8::instructions::Chip8;

pub fn decode(op: u16, i: &mut dyn Chip8) {
    match op & 0xF000 {
        0x0000 => match op {
            0x00E0 => i.cls(),
            0x00EE => i.ret(),
            _ => i.unknown(op),
        },
        0x1000 => i.jp_addr(op & 0x0FFF),
        0x2000 => i.call_addr(op & 0x0FFF),
        0x3000 => {
            let x = ((op & 0x0F00) >> 8) as u8;
            let byte = (op & 0x00FF) as u8;
            i.se_vx_byte(x, byte);
        }
        0x4000 => {
            let x = ((op & 0x0F00) >> 8) as u8;
            let byte = (op & 0x00FF) as u8;
            i.sne_vx_byte(x, byte);
        }
        0x5000 => {
            let x = ((op & 0x0F00) >> 8) as u8;
            let y = ((op & 0x00F0) >> 4) as u8;
            i.se_vx_vy(x, y);
        }
        0x6000 => {
            let x = ((op & 0x0F00) >> 8) as u8;
            let byte = (op & 0x00FF) as u8;
            i.ld_vx_byte(x, byte)
        }
        0x7000 => {
            let x = ((op & 0x0F00) >> 8) as u8;
            let byte = (op & 0x00FF) as u8;
            i.add_vx_byte(x, byte)
        }
        0x8000 => {
            let x = ((op & 0x0F00) >> 8) as u8;
            let y = ((op & 0x00F0) >> 4) as u8;
            match op & 0x000F {
                0x0000 => i.ld_vx_vy(x, y),
                0x0001 => i.or_vx_vy(x, y),
                0x0002 => i.and_vx_vy(x, y),
                0x0003 => i.xor_vx_vy(x, y),
                0x0004 => i.add_vx_vy(x, y),
                0x0005 => i.sub_vx_vy(x, y),
                0x0006 => i.shr_vx_vy(x, y),
                0x0007 => i.subn_vx_vy(x, y),
                0x000E => i.shl_vx_vy(x, y),
                _ => i.unknown(op),
            }
        }
        0x9000 => {
            let x = ((op & 0x0F00) >> 8) as u8;
            let y = ((op & 0x00F0) >> 4) as u8;
            i.sne_vx_vy(x, y);
        }
        0xA000 => i.ld_i_addr(op & 0x0FFF),
        0xB000 => i.jp_v0_addr(op & 0x0FFF),
        0xC000 => {
            let x = ((op & 0x0F00) >> 8) as u8;
            let byte = (op & 0x00FF) as u8;
            i.rnd_vx_byte(x, byte);
        }
        0xD000 => {
            let x = ((op & 0x0F00) >> 8) as u8;
            let y = ((op & 0x00F0) >> 4) as u8;
            let nibble = (op & 0x000F) as u16;
            i.drw_vx_vy_nibble(x, y, nibble);
        }
        0xE000 => {
            let x = ((op & 0x0F00) >> 8) as u8;
            match op & 0x00FF {
                0x009E => i.skp_vx(x),
                0x00A1 => i.sknp_vx(x),
                _ => i.unknown(op),
            }
        }
        0xF000 => {
            let x = ((op & 0x0F00) >> 8) as u8;
            match op & 0x00FF {
                0x0007 => i.ld_vx_dt(x),
                0x000A => i.ld_vx_k(x),
                0x0015 => i.ld_dt_vx(x),
                0x0018 => i.ld_st_vx(x),
                0x001E => i.add_i_vx(x),
                0x0029 => i.ld_f_vx(x),
                0x0033 => i.ld_b_vx(x),
                0x0055 => i.ld_i_vx(x),
                0x0065 => i.ld_vx_i(x),
                _ => i.unknown(op),
            }
        }
        _ => i.unknown(op),
    }
}

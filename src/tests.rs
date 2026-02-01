use super::*;

fn setup_cpu_with_opcode(opcode: u16) -> Chip8 {
    let mut cpu = Chip8::new();
    cpu.memory[PROGRAM_START] = (opcode >> 8) as u8;
    cpu.memory[PROGRAM_START + 1] = (opcode & 0xFF) as u8;
    cpu
}

#[test]
fn font_set_loads_into_memory() {
    let cpu = Chip8::new();
    assert_eq!(&cpu.memory[..FONT_SET_SIZE], &FONT_SET[..]);
}

#[test]
fn loads_rom_into_memory() {
    let mut cpu = Chip8::new();
    cpu.load_rom("chip8-test-rom/test_opcode.ch8").unwrap();
    let rom = fs::read("chip8-test-rom/test_opcode.ch8").unwrap();
    assert_eq!(
        &cpu.memory[PROGRAM_START..PROGRAM_START + rom.len()],
        &rom[..]
    );
}

#[test]
fn cls_clears_display() {
    let mut cpu = Chip8::new();
    cpu.display.iter_mut().for_each(|p| *p = true);
    cpu.memory[PROGRAM_START] = 0x00;
    cpu.memory[PROGRAM_START + 1] = 0xE0;
    cpu.cycle();
    assert!(cpu.display.iter().all(|&p| !p));
}

#[test]
fn ret_returns_from_subroutine() {
    let mut cpu = Chip8::new();
    cpu.pc = 0x300;
    cpu.stack[0] = 0x200;
    cpu.sp = 1;
    cpu.memory[0x300] = 0x00;
    cpu.memory[0x301] = 0xEE;
    cpu.cycle();
    assert_eq!(cpu.pc, 0x200);
    assert_eq!(cpu.sp, 0);
}

#[test]
fn jp_jumps_to_address() {
    let mut cpu = setup_cpu_with_opcode(0x1123);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x123);
}

#[test]
fn call_stores_pc_and_jumps() {
    let mut cpu = setup_cpu_with_opcode(0x2123);
    cpu.cycle();
    assert_eq!(cpu.sp, 1);
    assert_eq!(cpu.stack[0], PROGRAM_START as u16 + 2);
    assert_eq!(cpu.pc, 0x123);
}

#[test]
fn se_vx_byte_skips_when_equal() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x42;
    cpu.memory[PROGRAM_START] = 0x30;
    cpu.memory[PROGRAM_START + 1] = 0x42;
    cpu.cycle();
    assert_eq!(cpu.pc, PROGRAM_START as u16 + 4);
}

#[test]
fn se_vx_byte_no_skip_when_not_equal() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x42;
    cpu.memory[PROGRAM_START] = 0x30;
    cpu.memory[PROGRAM_START + 1] = 0x43;
    cpu.cycle();
    assert_eq!(cpu.pc, PROGRAM_START as u16 + 2);
}

#[test]
fn sne_vx_byte_skips_when_not_equal() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x42;
    cpu.memory[PROGRAM_START] = 0x40;
    cpu.memory[PROGRAM_START + 1] = 0x43;
    cpu.cycle();
    assert_eq!(cpu.pc, PROGRAM_START as u16 + 4);
}

#[test]
fn sne_vx_byte_no_skip_when_equal() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x42;
    cpu.memory[PROGRAM_START] = 0x40;
    cpu.memory[PROGRAM_START + 1] = 0x42;
    cpu.cycle();
    assert_eq!(cpu.pc, PROGRAM_START as u16 + 2);
}

#[test]
fn se_vx_vy_skips_when_equal() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x42;
    cpu.registers[1] = 0x42;
    cpu.memory[PROGRAM_START] = 0x50;
    cpu.memory[PROGRAM_START + 1] = 0x10;
    cpu.cycle();
    assert_eq!(cpu.pc, PROGRAM_START as u16 + 4);
}

#[test]
fn se_vx_vy_no_skip_when_not_equal() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x42;
    cpu.registers[1] = 0x43;
    cpu.memory[PROGRAM_START] = 0x50;
    cpu.memory[PROGRAM_START + 1] = 0x10;
    cpu.cycle();
    assert_eq!(cpu.pc, PROGRAM_START as u16 + 2);
}

#[test]
fn ld_vx_byte() {
    let mut cpu = setup_cpu_with_opcode(0x6042);
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x42);
}

#[test]
fn add_vx_byte() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x10;
    cpu.memory[PROGRAM_START] = 0x70;
    cpu.memory[PROGRAM_START + 1] = 0x20;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x30);
}

#[test]
fn ld_vx_vy() {
    let mut cpu = Chip8::new();
    cpu.registers[1] = 0x42;
    cpu.memory[PROGRAM_START] = 0x80;
    cpu.memory[PROGRAM_START + 1] = 0x10;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x42);
}

#[test]
fn or_vx_vy() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x01;
    cpu.registers[1] = 0x02;
    cpu.memory[PROGRAM_START] = 0x80;
    cpu.memory[PROGRAM_START + 1] = 0x11;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x03);
}

#[test]
fn and_vx_vy() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x03;
    cpu.registers[1] = 0x01;
    cpu.memory[PROGRAM_START] = 0x80;
    cpu.memory[PROGRAM_START + 1] = 0x12;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x01);
}

#[test]
fn xor_vx_vy() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x03;
    cpu.registers[1] = 0x01;
    cpu.memory[PROGRAM_START] = 0x80;
    cpu.memory[PROGRAM_START + 1] = 0x13;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x02);
}

#[test]
fn add_vx_vy_no_carry() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x10;
    cpu.registers[1] = 0x20;
    cpu.memory[PROGRAM_START] = 0x80;
    cpu.memory[PROGRAM_START + 1] = 0x14;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x30);
    assert_eq!(cpu.registers[0xF], 0);
}

#[test]
fn add_vx_vy_with_carry() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0xFF;
    cpu.registers[1] = 0x01;
    cpu.memory[PROGRAM_START] = 0x80;
    cpu.memory[PROGRAM_START + 1] = 0x14;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x00);
    assert_eq!(cpu.registers[0xF], 1);
}

#[test]
fn sub_vx_vy_no_borrow() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x30;
    cpu.registers[1] = 0x10;
    cpu.memory[PROGRAM_START] = 0x80;
    cpu.memory[PROGRAM_START + 1] = 0x15;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x20);
    assert_eq!(cpu.registers[0xF], 1);
}

#[test]
fn sub_vx_vy_with_borrow() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x10;
    cpu.registers[1] = 0x30;
    cpu.memory[PROGRAM_START] = 0x80;
    cpu.memory[PROGRAM_START + 1] = 0x15;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0xE0);
    assert_eq!(cpu.registers[0xF], 0);
}

#[test]
fn shr_vx_vy() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x06;
    cpu.memory[PROGRAM_START] = 0x80;
    cpu.memory[PROGRAM_START + 1] = 0x06;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x03);
    assert_eq!(cpu.registers[0xF], 0);
}

#[test]
fn shr_vx_vy_lsb_1() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x07;
    cpu.memory[PROGRAM_START] = 0x80;
    cpu.memory[PROGRAM_START + 1] = 0x06;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x03);
    assert_eq!(cpu.registers[0xF], 1);
}

#[test]
fn subn_vx_vy() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x10;
    cpu.registers[1] = 0x30;
    cpu.memory[PROGRAM_START] = 0x80;
    cpu.memory[PROGRAM_START + 1] = 0x17;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x20);
    assert_eq!(cpu.registers[0xF], 1);
}

#[test]
fn shl_vx_vy() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x03;
    cpu.memory[PROGRAM_START] = 0x80;
    cpu.memory[PROGRAM_START + 1] = 0x0E;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x06);
    assert_eq!(cpu.registers[0xF], 0);
}

#[test]
fn shl_vx_vy_msb_1() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x80;
    cpu.memory[PROGRAM_START] = 0x80;
    cpu.memory[PROGRAM_START + 1] = 0x0E;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x00);
    assert_eq!(cpu.registers[0xF], 1);
}

#[test]
fn sne_vx_vy() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x42;
    cpu.registers[1] = 0x43;
    cpu.memory[PROGRAM_START] = 0x90;
    cpu.memory[PROGRAM_START + 1] = 0x10;
    cpu.cycle();
    assert_eq!(cpu.pc, PROGRAM_START as u16 + 4);
}

#[test]
fn ld_i_addr() {
    let mut cpu = setup_cpu_with_opcode(0xA123);
    cpu.cycle();
    assert_eq!(cpu.index, 0x123);
}

#[test]
fn jp_v0_addr() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x10;
    cpu.memory[PROGRAM_START] = 0xB1;
    cpu.memory[PROGRAM_START + 1] = 0x23;
    cpu.cycle();
    assert_eq!(cpu.pc, 0x133);
}

#[test]
fn rnd_vx_byte() {
    let mut cpu = setup_cpu_with_opcode(0xC0FF);
    cpu.cycle();
    // Just verify it sets a value, exact value depends on RNG
    assert_eq!(cpu.pc, PROGRAM_START as u16 + 2);
}

#[test]
fn drw_vx_vy_nibble() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0;
    cpu.registers[1] = 0;
    cpu.index = 0x300;
    cpu.memory[0x300] = 0xFF;
    cpu.memory[PROGRAM_START] = 0xD0;
    cpu.memory[PROGRAM_START + 1] = 0x11;
    cpu.cycle();
    assert!(cpu.display[0]);
    assert!(cpu.display[7]);
    assert_eq!(cpu.registers[0xF], 0);
}

#[test]
fn drw_vx_vy_collision() {
    let mut cpu = Chip8::new();
    cpu.display[0] = true;
    cpu.registers[0] = 0;
    cpu.registers[1] = 0;
    cpu.index = 0x300;
    cpu.memory[0x300] = 0x80;
    cpu.memory[PROGRAM_START] = 0xD0;
    cpu.memory[PROGRAM_START + 1] = 0x11;
    cpu.cycle();
    assert!(!cpu.display[0]);
    assert_eq!(cpu.registers[0xF], 1);
}

#[test]
fn skp_vx_skips_when_pressed() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x0A;
    cpu.keys[0x0A] = true;
    cpu.memory[PROGRAM_START] = 0xE0;
    cpu.memory[PROGRAM_START + 1] = 0x9E;
    cpu.cycle();
    assert_eq!(cpu.pc, PROGRAM_START as u16 + 4);
}

#[test]
fn skp_vx_no_skip_when_not_pressed() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x0A;
    cpu.keys[0x0A] = false;
    cpu.memory[PROGRAM_START] = 0xE0;
    cpu.memory[PROGRAM_START + 1] = 0x9E;
    cpu.cycle();
    assert_eq!(cpu.pc, PROGRAM_START as u16 + 2);
}

#[test]
fn sknp_vx_skips_when_not_pressed() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x0A;
    cpu.keys[0x0A] = false;
    cpu.memory[PROGRAM_START] = 0xE0;
    cpu.memory[PROGRAM_START + 1] = 0xA1;
    cpu.cycle();
    assert_eq!(cpu.pc, PROGRAM_START as u16 + 4);
}

#[test]
fn sknp_vx_no_skip_when_pressed() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x0A;
    cpu.keys[0x0A] = true;
    cpu.memory[PROGRAM_START] = 0xE0;
    cpu.memory[PROGRAM_START + 1] = 0xA1;
    cpu.cycle();
    assert_eq!(cpu.pc, PROGRAM_START as u16 + 2);
}

#[test]
fn ld_vx_dt() {
    let mut cpu = Chip8::new();
    cpu.delay_timer = 0x42;
    cpu.memory[PROGRAM_START] = 0xF0;
    cpu.memory[PROGRAM_START + 1] = 0x07;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x42);
}

#[test]
fn ld_dt_vx() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x42;
    cpu.memory[PROGRAM_START] = 0xF0;
    cpu.memory[PROGRAM_START + 1] = 0x15;
    cpu.cycle();
    assert_eq!(cpu.delay_timer, 0x42);
}

#[test]
fn ld_st_vx() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x42;
    cpu.memory[PROGRAM_START] = 0xF0;
    cpu.memory[PROGRAM_START + 1] = 0x18;
    cpu.cycle();
    assert_eq!(cpu.sound_timer, 0x42);
}

#[test]
fn add_i_vx() {
    let mut cpu = Chip8::new();
    cpu.index = 0x100;
    cpu.registers[0] = 0x42;
    cpu.memory[PROGRAM_START] = 0xF0;
    cpu.memory[PROGRAM_START + 1] = 0x1E;
    cpu.cycle();
    assert_eq!(cpu.index, 0x142);
}

#[test]
fn ld_f_vx() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x0A;
    cpu.memory[PROGRAM_START] = 0xF0;
    cpu.memory[PROGRAM_START + 1] = 0x29;
    cpu.cycle();
    assert_eq!(cpu.index, 0x32);
}

#[test]
fn ld_b_vx() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 137;
    cpu.index = 0x500;
    cpu.memory[PROGRAM_START] = 0xF0;
    cpu.memory[PROGRAM_START + 1] = 0x33;
    cpu.cycle();
    assert_eq!(cpu.memory[0x500], 1);
    assert_eq!(cpu.memory[0x501], 3);
    assert_eq!(cpu.memory[0x502], 7);
}

#[test]
fn ld_i_vx() {
    let mut cpu = Chip8::new();
    cpu.registers[0] = 0x01;
    cpu.registers[1] = 0x02;
    cpu.registers[2] = 0x03;
    cpu.index = 0x500;
    cpu.memory[PROGRAM_START] = 0xF2;
    cpu.memory[PROGRAM_START + 1] = 0x55;
    cpu.cycle();
    assert_eq!(cpu.memory[0x500], 0x01);
    assert_eq!(cpu.memory[0x501], 0x02);
    assert_eq!(cpu.memory[0x502], 0x03);
}

#[test]
fn ld_vx_i() {
    let mut cpu = Chip8::new();
    cpu.index = 0x500;
    cpu.memory[0x500] = 0x01;
    cpu.memory[0x501] = 0x02;
    cpu.memory[0x502] = 0x03;
    cpu.memory[PROGRAM_START] = 0xF2;
    cpu.memory[PROGRAM_START + 1] = 0x65;
    cpu.cycle();
    assert_eq!(cpu.registers[0], 0x01);
    assert_eq!(cpu.registers[1], 0x02);
    assert_eq!(cpu.registers[2], 0x03);
}

#[test]
fn update_timers_decrements() {
    let mut cpu = Chip8::new();
    cpu.delay_timer = 5;
    cpu.sound_timer = 3;
    cpu.update_timers();
    assert_eq!(cpu.delay_timer, 4);
    assert_eq!(cpu.sound_timer, 2);
}

#[test]
fn update_timers_stops_at_zero() {
    let mut cpu = Chip8::new();
    cpu.delay_timer = 0;
    cpu.sound_timer = 0;
    cpu.update_timers();
    assert_eq!(cpu.delay_timer, 0);
    assert_eq!(cpu.sound_timer, 0);
}

#[test]
fn ld_vx_k_waits_for_key() {
    let mut cpu = Chip8::new();
    cpu.memory[PROGRAM_START] = 0xF0;
    cpu.memory[PROGRAM_START + 1] = 0x0A;
    cpu.cycle();
    assert!(cpu.waiting_for_key.is_some());
    assert_eq!(cpu.pc, PROGRAM_START as u16 + 2);
}

#[test]
fn ld_vx_k_stores_key_when_pressed() {
    let mut cpu = Chip8::new();
    cpu.memory[PROGRAM_START] = 0xF0;
    cpu.memory[PROGRAM_START + 1] = 0x0A;
    cpu.cycle();
    assert!(cpu.waiting_for_key.is_some());
    cpu.keys[5] = true;
    cpu.cycle();
    assert!(cpu.waiting_for_key.is_none());
    assert_eq!(cpu.registers[0], 5);
}

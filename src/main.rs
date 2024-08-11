//use raylib::prelude::*;

use std::{fs, path::Path};

#[derive(Debug)]
#[allow(dead_code)] //TODO: remove
pub struct Chip8 {
    memory: [u8; 4095],
    graphics: [u8; 64 * 32],
    registers: [u8; 16],
    index: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    keys: [u8; 16],
}

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const PROG_MEM_MIN: usize = 0x200;
const PROG_MEM_MAX: usize = 0x600;

#[allow(dead_code)] //TODO: remove
impl Chip8 {
    fn new() -> Self {
        let mut memory = [0; 4095];

        memory[..FONT_SET.len()].copy_from_slice(&FONT_SET);

        Self {
            memory,
            graphics: [0; 64 * 32],
            registers: [0; 16],
            index: 0,
            program_counter: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keys: [0; 16],
        }
    }
    fn increment_pc(&mut self) {
        self.program_counter += 2
    }
    fn cycle(&mut self) {
        let opcode = (self.memory[self.program_counter as usize] as u16) << 8;
        let operands = self.memory[self.program_counter as usize + 1] as u16;
        let instruction = opcode | operands;

        let msb = self.memory[self.program_counter as usize] >> 4;

        match msb {
            0x0 => {
                match instruction {
                    0x00E0 => self.graphics.iter_mut().for_each(|pixel| *pixel = 0),
                    0x00EE => {
                        self.program_counter = self.stack[self.sp as usize];
                        self.sp -= 1;
                    }
                    _ => panic!("SYS Instructions are not handled!"),
                }
                self.increment_pc()
            }
            0x1 => todo!(),
            _ => unreachable!(),
        }
    }
    fn load_rom<P: AsRef<Path>>(&mut self, file: P) {
        let rom = fs::read(file).expect("File not found.");
        rom.iter()
            .enumerate()
            .take_while(|(i, _)| (*i + PROG_MEM_MIN) < PROG_MEM_MAX)
            .for_each(|(i, &b)| self.memory[i + PROG_MEM_MIN] = b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_set_loads_into_memory() {
        let cpu = Chip8::new();
        cpu.memory
            .iter()
            .enumerate()
            .take_while(|(i, _)| *i < FONT_SET.len() && *i < PROG_MEM_MIN)
            .for_each(|(i, &b)| assert_eq!(b, FONT_SET[i]));
    }
    #[test]
    fn loads_rom_into_memory() {
        let mut cpu = Chip8::new();
        let path = "chip8-test-rom/test_opcode.ch8";
        let rom_bytes = fs::read(path).expect("File not found.");

        cpu.load_rom(path.to_string());

        rom_bytes
            .iter()
            .enumerate()
            .for_each(|(i, &b)| assert_eq!(b, cpu.memory[PROG_MEM_MIN + i]));
    }
}

fn main() {
    let mut cpu = Chip8::new();
    cpu.load_rom("chip8-test-rom/test_opcode.ch8");

    //TODO: load ROM
    /*
        let (mut rl, thread) = raylib::init()
            .size(640, 480)
            .title("Chip8 Emulator")
            .build();

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::BLACK);
            //TODO: Scale and display graphics
        }
    */
}

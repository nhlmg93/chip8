//use raylib::prelude::*;

#[derive(Debug)]
#[allow(dead_code)] //TODO: remove
pub struct Chip8 {
    opcode: u8,
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

impl Chip8 {
    fn new() -> Self {
        let mut memory = [0; 4095];

        memory[..FONT_SET.len()].copy_from_slice(&FONT_SET);

        Self {
            opcode: 0,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_ITERPRETER_MEMORY: usize = 0x200;

    #[test]
    fn font_set_loads_into_memory() {
        let cpu = Chip8::new();
        cpu.memory
            .iter()
            .enumerate()
            .take_while(|(i, _)| *i < FONT_SET.len() && *i < MAX_ITERPRETER_MEMORY)
            .for_each(|(i, &b)| assert_eq!(b, FONT_SET[i]));
    }
}

fn main() {
    let _cpu = Chip8::new();

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

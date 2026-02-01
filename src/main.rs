use raylib::prelude::*;
use std::fs;
use std::path::Path;

const MEMORY_SIZE: usize = 4096;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const FONT_SET_SIZE: usize = 80;
const PROGRAM_START: usize = 0x200;

const FONT_SET: [u8; FONT_SET_SIZE] = [
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

pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    registers: [u8; NUM_REGISTERS],
    index: u16,
    pc: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; STACK_SIZE],
    sp: u8,
    keys: [bool; NUM_KEYS],
    draw_flag: bool,
    waiting_for_key: Option<usize>,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

impl Chip8 {
    pub fn new() -> Self {
        let mut memory = [0; MEMORY_SIZE];
        memory[..FONT_SET_SIZE].copy_from_slice(&FONT_SET);

        Self {
            memory,
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            registers: [0; NUM_REGISTERS],
            index: 0,
            pc: PROGRAM_START as u16,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            keys: [false; NUM_KEYS],
            draw_flag: false,
            waiting_for_key: None,
        }
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let rom = fs::read(path).map_err(|e| format!("Failed to read ROM: {}", e))?;
        let len = rom.len().min(MEMORY_SIZE - PROGRAM_START);
        self.memory[PROGRAM_START..PROGRAM_START + len].copy_from_slice(&rom[..len]);
        Ok(())
    }

    pub fn set_key(&mut self, key: usize, pressed: bool) {
        if key < NUM_KEYS {
            self.keys[key] = pressed;
        }
    }

    pub fn get_display(&self) -> &[bool] {
        &self.display
    }

    pub fn should_draw(&self) -> bool {
        self.draw_flag
    }

    pub fn clear_draw_flag(&mut self) {
        self.draw_flag = false;
    }

    pub fn cycle(&mut self) {
        if let Some(vx) = self.waiting_for_key {
            for (i, &pressed) in self.keys.iter().enumerate() {
                if pressed {
                    self.registers[vx] = i as u8;
                    self.waiting_for_key = None;
                    self.pc += 2;
                    break;
                }
            }
            return;
        }

        let opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[self.pc as usize + 1] as u16);
        self.pc += 2;

        let nibble = |shift: u16| ((opcode >> shift) & 0xF) as usize;
        let byte = || (opcode & 0xFF) as u8;
        let addr = || opcode & 0xFFF;

        match (opcode >> 12, nibble(8), nibble(4), nibble(0)) {
            (0x0, 0x0, 0xE, 0x0) => self.display.iter_mut().for_each(|p| *p = false),
            (0x0, 0x0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            (0x1, _, _, _) => self.pc = addr(),
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = addr();
            }
            (0x3, x, _, _) => {
                if self.registers[x] == byte() {
                    self.pc += 2;
                }
            }
            (0x4, x, _, _) => {
                if self.registers[x] != byte() {
                    self.pc += 2;
                }
            }
            (0x5, x, y, 0x0) => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
            }
            (0x6, x, _, _) => self.registers[x] = byte(),
            (0x7, x, _, _) => self.registers[x] = self.registers[x].wrapping_add(byte()),
            (0x8, x, y, 0x0) => self.registers[x] = self.registers[y],
            (0x8, x, y, 0x1) => self.registers[x] |= self.registers[y],
            (0x8, x, y, 0x2) => self.registers[x] &= self.registers[y],
            (0x8, x, y, 0x3) => self.registers[x] ^= self.registers[y],
            (0x8, x, y, 0x4) => {
                let (res, carry) = self.registers[x].overflowing_add(self.registers[y]);
                self.registers[0xF] = carry as u8;
                self.registers[x] = res;
            }
            (0x8, x, y, 0x5) => {
                self.registers[0xF] = (self.registers[x] >= self.registers[y]) as u8;
                self.registers[x] = self.registers[x].wrapping_sub(self.registers[y]);
            }
            (0x8, x, _, 0x6) => {
                self.registers[0xF] = self.registers[x] & 1;
                self.registers[x] >>= 1;
            }
            (0x8, x, y, 0x7) => {
                self.registers[0xF] = (self.registers[y] >= self.registers[x]) as u8;
                self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]);
            }
            (0x8, x, _, 0xE) => {
                self.registers[0xF] = (self.registers[x] >> 7) & 1;
                self.registers[x] <<= 1;
            }
            (0x9, x, y, 0x0) => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => self.index = addr(),
            (0xB, _, _, _) => self.pc = addr() + self.registers[0] as u16,
            (0xC, x, _, _) => self.registers[x] = rand::random::<u8>() & byte(),
            (0xD, x, y, n) => self.draw(x, y, n),
            (0xE, x, 0x9, 0xE) => {
                if self.keys[self.registers[x] as usize & 0xF] {
                    self.pc += 2;
                }
            }
            (0xE, x, 0xA, 0x1) => {
                if !self.keys[self.registers[x] as usize & 0xF] {
                    self.pc += 2;
                }
            }
            (0xF, x, 0x0, 0x7) => self.registers[x] = self.delay_timer,
            (0xF, x, 0x0, 0xA) => self.waiting_for_key = Some(x),
            (0xF, x, 0x1, 0x5) => self.delay_timer = self.registers[x],
            (0xF, x, 0x1, 0x8) => self.sound_timer = self.registers[x],
            (0xF, x, 0x1, 0xE) => {
                let (new_index, overflow) = self.index.overflowing_add(self.registers[x] as u16);
                self.index = new_index;
                if overflow {
                    self.registers[0xF] = 1;
                }
            }
            (0xF, x, 0x2, 0x9) => self.index = (self.registers[x] as u16 & 0xF) * 5,
            (0xF, x, 0x3, 0x3) => {
                let val = self.registers[x];
                self.memory[self.index as usize] = val / 100;
                self.memory[self.index as usize + 1] = (val / 10) % 10;
                self.memory[self.index as usize + 2] = val % 10;
            }
            (0xF, x, 0x5, 0x5) => {
                for i in 0..=x {
                    self.memory[self.index as usize + i] = self.registers[i];
                }
            }
            (0xF, x, 0x6, 0x5) => {
                for i in 0..=x {
                    self.registers[i] = self.memory[self.index as usize + i];
                }
            }
            _ => panic!(
                "Unknown opcode: 0x{:04X} at PC: 0x{:04X}",
                opcode,
                self.pc - 2
            ),
        }
    }

    fn draw(&mut self, x: usize, y: usize, n: usize) {
        let x_pos = self.registers[x] as usize % DISPLAY_WIDTH;
        let y_pos = self.registers[y] as usize % DISPLAY_HEIGHT;
        self.registers[0xF] = 0;

        for row in 0..n {
            let sprite = self.memory[self.index as usize + row];
            let py = (y_pos + row) % DISPLAY_HEIGHT;
            for col in 0..8 {
                let px = (x_pos + col) % DISPLAY_WIDTH;
                let pixel = (sprite >> (7 - col)) & 1;
                if pixel == 1 {
                    let idx = py * DISPLAY_WIDTH + px;
                    if self.display[idx] {
                        self.registers[0xF] = 1;
                    }
                    self.display[idx] ^= true;
                }
            }
        }
        self.draw_flag = true;
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}

const SCALE: usize = 15;
const WINDOW_WIDTH: i32 = (DISPLAY_WIDTH * SCALE) as i32;
const WINDOW_HEIGHT: i32 = (DISPLAY_HEIGHT * SCALE) as i32;
const CYCLES_PER_FRAME: usize = 10;

fn get_chip8_key(key: KeyboardKey) -> Option<usize> {
    match key {
        KeyboardKey::KEY_X => Some(0x0),
        KeyboardKey::KEY_ONE => Some(0x1),
        KeyboardKey::KEY_TWO => Some(0x2),
        KeyboardKey::KEY_THREE => Some(0x3),
        KeyboardKey::KEY_Q => Some(0x4),
        KeyboardKey::KEY_W => Some(0x5),
        KeyboardKey::KEY_E => Some(0x6),
        KeyboardKey::KEY_A => Some(0x7),
        KeyboardKey::KEY_S => Some(0x8),
        KeyboardKey::KEY_D => Some(0x9),
        KeyboardKey::KEY_Z => Some(0xA),
        KeyboardKey::KEY_C => Some(0xB),
        KeyboardKey::KEY_FOUR => Some(0xC),
        KeyboardKey::KEY_R => Some(0xD),
        KeyboardKey::KEY_F => Some(0xE),
        KeyboardKey::KEY_V => Some(0xF),
        _ => None,
    }
}

fn main() {
    let mut cpu = Chip8::new();
    if let Err(e) = cpu.load_rom("chip8-test-rom/test_opcode.ch8") {
        eprintln!("Error: {}", e);
        return;
    }

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("CHIP-8 Emulator")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // Handle input
        if let Some(key) = rl.get_key_pressed() {
            if let Some(chip8_key) = get_chip8_key(key) {
                cpu.set_key(chip8_key, true);
            }
        }

        // Check for key releases (poll all mapped keys)
        let mapped_keys = [
            KeyboardKey::KEY_X,
            KeyboardKey::KEY_ONE,
            KeyboardKey::KEY_TWO,
            KeyboardKey::KEY_THREE,
            KeyboardKey::KEY_Q,
            KeyboardKey::KEY_W,
            KeyboardKey::KEY_E,
            KeyboardKey::KEY_A,
            KeyboardKey::KEY_S,
            KeyboardKey::KEY_D,
            KeyboardKey::KEY_Z,
            KeyboardKey::KEY_C,
            KeyboardKey::KEY_FOUR,
            KeyboardKey::KEY_R,
            KeyboardKey::KEY_F,
            KeyboardKey::KEY_V,
        ];
        for key in mapped_keys.iter() {
            if let Some(chip8_key) = get_chip8_key(*key) {
                cpu.set_key(chip8_key, rl.is_key_down(*key));
            }
        }

        // Run CPU cycles
        for _ in 0..CYCLES_PER_FRAME {
            cpu.cycle();
        }

        // Update timers at 60Hz (once per frame)
        cpu.update_timers();

        // Render
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let display = cpu.get_display();
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let idx = y * DISPLAY_WIDTH + x;
                if display[idx] {
                    let screen_x = (x * SCALE) as i32;
                    let screen_y = (y * SCALE) as i32;
                    let width = SCALE as i32;
                    let height = SCALE as i32;
                    d.draw_rectangle(screen_x, screen_y, width, height, Color::WHITE);
                }
            }
        }

        cpu.clear_draw_flag();
    }
}

#[cfg(test)]
mod tests;

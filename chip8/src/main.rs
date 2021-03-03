//! CHIP-8 emulator in pure Rust.

mod display;
mod font;
mod keyboard;
mod timer;

use crate::display::Display;
use crate::font::FONT_SET;
use crate::keyboard::Keyboard;
use crate::timer::Timer;
use std::time::Duration;
use tinyvec::*;
use tokio::time;

/// CHIP-8 implementation in Rust.
pub struct CHIP8 {
    /// Memory for the CHIP-8.
    ///
    /// All memory is considered RAM and therefore writable. We use 4KB
    /// (4096 bytes) because the index register and program counter can only address 12 bits (4096
    /// addresses).
    ///
    /// The first 512 bits are reserved (0x000..0x200).
    pub memory: [u8; 4096],

    /// Display for the CHIP-8.
    pub display: Display,

    /// Stack for the CHIP-8.
    ///
    /// The stack is comprised of 16 two-byte entries. In case of overflow, data will
    /// be allocated on the heap.
    pub stack: TinyVec<[(u8, u8); 16]>,

    /// Variable register for the CHIP-8.
    ///
    /// These are numbered `0x0-0xF` and referenced as V0-VF. `VF` is also used as a flag register
    /// based on some rule.
    pub variable: [u8; 16],

    /// Index register for the CHIP-8.
    ///
    /// The index register points to locations in memory.
    pub index: u16,

    /// Timers for the CHIP-8.
    pub timer: Timer,

    /// Keyboard for the CHIP-8.
    ///
    /// TODO: Implement keypad somewhere here.
    pub keyboard: Keyboard,

    /// Program Counter for the CHIP-8.
    ///
    /// The program counter points to the current instruction in memory.
    pub program_counter: u16,
}

impl CHIP8 {
    /// Constructs a new, empty `CHIP8`.
    ///
    /// # Examples
    /// ```
    /// let mut chip: CHIP8 = CHIP8::new();
    /// ```
    pub fn new() -> Self {
        let mut memory: [u8; 4096] = [0; 4096];
        // Insert fonts into address range 0x50..=0x9F.
        for address in 0x50..=0x9F {
            let font_idx = address - 0x50;
            memory[address] = FONT_SET[font_idx];
        }

        let display: Display = Display::new();

        let stack: TinyVec<[(u8, u8); 16]> = tiny_vec!();

        let variable: [u8; 16] = [0; 16];

        let index: u16 = 0;

        let timer: Timer = Timer::new();

        let keyboard: Keyboard = Keyboard::new();

        let program_counter: u16 = 0;

        CHIP8 {
            memory,
            display,
            stack,
            variable,
            index,
            timer,
            keyboard,
            program_counter,
        }
    }

    /// Fetches an instruction from the current program counter.
    pub fn fetch(&mut self) -> (u8, u8) {
        let instr_one = self.memory[self.program_counter as usize];
        self.program_counter += 1;

        let instr_two = self.memory[self.program_counter as usize];
        self.program_counter += 1;

        (instr_one, instr_two)
    }

    /// Decodes and executes the given instruction.
    pub fn decode_execute(&mut self, instruction: (u8, u8)) {
        // nibble 1. type of instruction.
        let itype: u8 = (instruction.0 & 0xF0) >> 4;
        // nibble 2. Used to look up one of 16 registers V0-VF.
        let x: u8 = instruction.0 & 0x0F;
        // nibble 3. Used to look up one of 16 registers V0-VF.
        let y: u8 = (instruction.1 & 0xF0) >> 4;
        // nibble 4. 4-bit number.
        let n: u8 = instruction.1 & 0x0F;
        // second byte (nibble 3 and 4). 8-bit immediate number.
        let nn: u8 = instruction.1;
        // nibble 2, 3, and 4. 12-bit immediate memory address
        let nnn: u16 = (x as u16) << 8 | (y as u16) << 4 | (n as u16);

        match itype {
            // 00E0 - Clear screen.
            0x0 => self.display.clear(),
            // 1NNN - Jump. Set the program counter to nnn.
            0x1 => match nnn {
                _ => self.program_counter = nnn,
            },
            // 6XNN - Set register VX to nn.
            0x6 => self.variable[x as usize] = nn,
            // 7XNN - Add value nn to VX.
            0x7 => {
                // TODO: Overflow???
                if self.variable[0] == 0xFF {
                    self.variable[x as usize] += nn;
                }
            }
            // ANNN - Set index register I to nnn.
            0xA => self.index = nnn,
            // DXYN - Display/Draw
            // TODO: Call a method in Display to handle this.
            0xD => println!("D"),
            _ => panic!("error: unknown instruction {:x}", itype),
        };
    }

    /// Runs the emulator.
    ///
    /// The emulator runs at a speed of 700 instructions per second (700 Hz).
    pub async fn run(&mut self) {
        // 700 instructions per second
        let interval = time::interval(Duration::from_micros(1429));
        tokio::pin!(interval);

        loop {
            interval.as_mut().tick().await;
            self.timer.cycle();
        }
    }
}

/// Emulator entry-point.
#[tokio::main]
async fn main() {
    let mut chip = CHIP8::new();
    chip.run().await;
}

mod chipset;
mod display;
mod keyboard;
mod memory;
mod opcode_processor;

use self::chipset::{Chip8Chipset, Chipset, PROGRAM_COUNTER_BOUNDARY};
use self::display::ConsoleDisplay;
use self::keyboard::ConsoleKeyboard;
use self::memory::{Memory, Registers, Stack};
use self::opcode_processor::Chip8OpCodesProcessor;
use std::thread::sleep;
use std::time::Duration;

pub struct Emulator {
    memory: Memory,
    stack: Stack,
    fontset: Fontset,
    registers: Registers,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            memory: Memory::new(),
            stack: Stack::new(),
            fontset: Fontset::new(),
            registers: Registers::new(),
        }
    }

    pub fn initialize(mut self, data: &[u8]) -> InitializedEmulator {
        self.load_fonts();
        self.load_program(data);

        InitializedEmulator {
            chipset: Box::new(Chip8Chipset::new(
                self.memory,
                self.stack,
                self.registers,
                Chip8OpCodesProcessor::new(),
                ConsoleDisplay::new(),
                ConsoleKeyboard::new(),
            )),
        }
    }

    fn load_fonts(&mut self) {
        for (address, font) in self.fontset.get_values().iter().enumerate() {
            self.memory.write(address as u16, *font);
        }
    }

    fn load_program(&mut self, data: &[u8]) {
        let mut address = PROGRAM_COUNTER_BOUNDARY;

        for byte in data {
            self.memory.write(address, *byte);
            address += 1;
        }
    }
}

pub struct InitializedEmulator {
    chipset: Box<dyn Chipset>,
}

impl InitializedEmulator {
    pub fn run(&mut self) {
        while let Ok(()) = self.chipset.tick() {
            sleep(Duration::from_millis(2));
        }
    }
}

struct Fontset {
    values: Vec<u8>,
}

impl Fontset {
    pub fn new() -> Fontset {
        Fontset {
            values: vec![
                0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
                0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
                0x90, 0x90, 0xf0, 0x10, 0x10, // 4
                0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
                0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
                0xf0, 0x10, 0x20, 0x40, 0x40, // 7
                0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
                0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
                0xf0, 0x90, 0xf0, 0x90, 0x90, // A
                0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
                0xf0, 0x80, 0x80, 0x80, 0xf0, // C
                0xe0, 0x90, 0x90, 0x90, 0xe0, // D
                0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
                0xf0, 0x80, 0xf0, 0x80, 0x80, // F
            ],
        }
    }

    pub fn get_values(&self) -> &Vec<u8> {
        &self.values
    }
}

#[cfg(test)]
mod test_emulator {
    use super::*;

    #[test]
    fn test_can_run_program() {
        let emulator = Emulator {
            memory: Memory::new(),
            stack: Stack::new(),
            fontset: Fontset::new(),
            registers: Registers::new(),
        };

        let mut initialized_emulator = emulator.initialize(&[0x00, 0xe0]);

        initialized_emulator.run();
    }
}

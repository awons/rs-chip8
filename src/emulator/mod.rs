mod memory;
mod chipset;
mod opcode_processor;
mod display;
mod keyboard;

use self::chipset::{Chipset, Chip8Chipset, PROGRAM_COUNTER_BOUNDARY};
use self::memory::{Memory, Stack, Registers};
use self::opcode_processor::OpCodesProcessor;
use self::display::Display;
use self::keyboard::Keyboard;

pub struct Emulator {
    memory: Memory,
    stack: Stack,
    fontset: Fontset,
    registers: Registers,
}

impl Emulator {
    pub fn initialize(mut self, data: &[u8]) -> InitializedEmulator {
        self.load_fonts();
        self.load_program(data);

        InitializedEmulator {
            chipset: Box::new(
                Chip8Chipset::new(
                    self.memory,
                    self.stack,
                    self.registers,
                    OpCodesProcessor::new(),
                    Display::new(),
                    Keyboard::new()
                )
            )
        }
    }

    fn load_fonts(&mut self) {
        let mut address: u16 = 0;
        for font in self.fontset.get_values() {
            self.memory.write(address, *font);
            address += 1;
        }
    }

    fn load_program(&mut self, data: &[u8])
    {
        let mut address = PROGRAM_COUNTER_BOUNDARY;

        for byte in data {
            self.memory.write(address, *byte);
            address += 1;
        }
    }
}

pub struct InitializedEmulator {
    chipset: Box<Chipset>,
}

impl InitializedEmulator {
    pub fn run(&mut self)
    {}
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
                0xf0, 0x80, 0xf0, 0x80, 0x80  // F
            ]
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
    fn test_can_initialize_chipset()
    {
        let emulator = Emulator {
            memory: Memory::new(),
            stack: Stack::new(),
            fontset: Fontset::new(),
            registers: Registers::new(),
        };

        let _ = emulator.initialize(&[0x1, 0x2, 0x3, 0x4, 0x5, 0x6]);
    }

    #[test]
    fn test_can_run_program() {
        let emulator = Emulator {
            memory: Memory::new(),
            stack: Stack::new(),
            fontset: Fontset::new(),
            registers: Registers::new(),
        };

        let mut initialized_emulator = emulator.initialize(&[0x1, 0x2, 0x3, 0x4, 0x5, 0x6]);

        initialized_emulator.run();
    }
}
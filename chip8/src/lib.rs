pub mod display;
pub mod keyboard;

mod chipset;
mod gpu;
mod memory;
mod opcode_processor;

use self::chipset::{Chip8Chipset, Chipset, PROGRAM_COUNTER_BOUNDARY};
use self::display::GraphicDisplay;
use self::gpu::Chip8Gpu;
use self::keyboard::Keyboard;
use self::memory::{Memory, Registers, Stack};
use self::opcode_processor::Chip8OpCodesProcessor;
use std::thread::sleep;
use std::time::Duration;

pub struct Emulator {
    memory: Memory,
    stack: Stack,
    fontset: Fontset,
    registers: Registers,
    opcode_processor: Chip8OpCodesProcessor,
    gpu: Chip8Gpu,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            memory: Memory::new(),
            stack: Stack::new(),
            fontset: Fontset::new(),
            registers: Registers::new(),
            opcode_processor: Chip8OpCodesProcessor::new(),
            gpu: Chip8Gpu::new(),
        }
    }

    pub fn initialize<'a, K, D>(
        mut self,
        data: &[u8],
        keyboard: K,
        display: D,
    ) -> InitializedEmulator<'a>
    where
        K: Keyboard + 'a,
        D: GraphicDisplay + 'a,
    {
        self.load_fonts();
        self.load_program(data);

        InitializedEmulator {
            chipset: Box::new(Chip8Chipset::new(
                self.memory,
                self.stack,
                self.registers,
                self.opcode_processor,
                self.gpu,
                keyboard,
                display,
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

pub struct InitializedEmulator<'a> {
    chipset: Box<dyn Chipset + 'a>,
}

impl<'a> InitializedEmulator<'a> {
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

    use crate::keyboard::Key;
    use std::ops;

    struct MockedKeyboard {}
    impl Keyboard for MockedKeyboard {
        fn get_pressed_key(&mut self) -> Option<Key> {
            None
        }

        fn wait_for_key_press(&mut self) -> Key {
            Key::Key0
        }
    }

    struct MocketDisplay {}
    impl GraphicDisplay for MocketDisplay {
        fn draw<M>(&mut self, _: &M)
        where
            M: ops::Index<usize, Output = [u8]>,
        {
        }
    }

    #[test]
    fn test_can_run_program() {
        let emulator = Emulator {
            memory: Memory::new(),
            stack: Stack::new(),
            fontset: Fontset::new(),
            registers: Registers::new(),
            gpu: Chip8Gpu::new(),
            opcode_processor: Chip8OpCodesProcessor::new(),
        };

        let mut initialized_emulator =
            emulator.initialize(&[0x00, 0xe0], MockedKeyboard {}, MocketDisplay {});

        initialized_emulator.run();
    }
}

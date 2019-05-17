pub mod chipset;
pub mod display;
pub mod gpu;
pub mod keyboard;
pub mod opcode_processor;

mod memory;

use chipset::PROGRAM_COUNTER_BOUNDARY;
use chipset::{Chip8Chipset, RandomByteGenerator};
use display::GraphicDisplay;
use gpu::{Chip8Gpu, Gpu};
use keyboard::Keyboard;
use memory::{Memory, Registers, Stack};
use opcode_processor::{Chip8OpCodesProcessor, OpCodesProcessor};
use std::result::Result;

pub struct Emulator {
    memory: Memory,
    stack: Stack,
    fontset: Fontset,
    registers: Registers,
    opcode_processor: Chip8OpCodesProcessor,
    gpu: Chip8Gpu,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            memory: Memory::new(),
            stack: Stack::new(),
            fontset: Fontset::new(),
            registers: Registers::new(),
            opcode_processor: Chip8OpCodesProcessor::new(),
            gpu: Chip8Gpu::new(),
        }
    }

    pub fn load_fonts(&mut self) {
        for (address, font) in self.fontset.get_values().iter().enumerate() {
            self.memory.write(address as u16, *font);
        }
    }

    pub fn load_program(&mut self, data: &[u8]) {
        let mut address = PROGRAM_COUNTER_BOUNDARY;

        for byte in data {
            self.memory.write(address, *byte);
            address += 1;
        }
    }

    pub fn initialize<K, D, R>(
        mut self,
        data: &[u8],
        keyboard: K,
        display: D,
        random_byte_generator: R,
    ) -> InitializedEmulator<Chip8OpCodesProcessor, Chip8Gpu, K, D, R>
    where
        K: Keyboard,
        D: GraphicDisplay,
        R: RandomByteGenerator,
    {
        self.load_fonts();
        self.load_program(data);

        InitializedEmulator {
            chipset: Chip8Chipset::new(
                self.memory,
                self.stack,
                self.registers,
                self.opcode_processor,
                self.gpu,
                keyboard,
                display,
                random_byte_generator,
            ),
        }
    }
}

pub struct InitializedEmulator<O, G, K, D, R>
where
    O: OpCodesProcessor,
    G: Gpu,
    K: Keyboard,
    D: GraphicDisplay,
    R: RandomByteGenerator,
{
    chipset: Chip8Chipset<O, G, K, D, R>,
}

impl<O, G, K, D, R> InitializedEmulator<O, G, K, D, R>
where
    O: OpCodesProcessor,
    G: Gpu,
    K: Keyboard,
    D: GraphicDisplay,
    R: RandomByteGenerator,
{
    pub fn run_cycle(&mut self) -> Result<(), String> {
        self.chipset.tick()
    }

    pub fn get_keyboard(&self) -> &K {
        self.chipset.get_keyboard()
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
    use super::{Emulator, Fontset};
    use crate::chipset::RandomByteGenerator;
    use crate::display::GraphicDisplay;
    use crate::gpu::Chip8Gpu;
    use crate::keyboard::{Key, Keyboard};
    use crate::memory::{Memory, Registers, Stack};
    use crate::opcode_processor::Chip8OpCodesProcessor;

    use rand;
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

    struct TestRandomByteGenerator {}
    impl RandomByteGenerator for TestRandomByteGenerator {
        fn generate(&self) -> u8 {
            rand::random::<u8>()
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

        let mut initialized_emulator = emulator.initialize(
            &[0x00, 0xe0],
            MockedKeyboard {},
            MocketDisplay {},
            TestRandomByteGenerator {},
        );

        while let Ok(()) = initialized_emulator.run_cycle() {}
    }
}

#[macro_use]
mod utils;
mod implementation;

use chip8::gpu::Chip8Gpu;
use chip8::opcode_processor::Chip8OpCodesProcessor;
use chip8::{Emulator, InitializedEmulator};
use implementation::display::BrowserDisplay;
use implementation::keyboard::BrowserKeyboard;
use implementation::random_byte_generator::RandRandomByteGenerator;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
    rom: Vec<u8>,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        utils::set_panic_hook();
        Game {
            rom: vec![0; 0xe00],
        }
    }

    pub fn get_rom_ptr(&self) -> *const u8 {
        self.rom.as_ptr()
    }

    pub fn start(&mut self) -> RunningGame {
        let emulator = Emulator::new();
        let keyboard = BrowserKeyboard::new();
        let display = BrowserDisplay::new();
        let random_byte_generator = RandRandomByteGenerator::new();

        let initialized_emulator =
            emulator.initialize(&self.rom, keyboard, display, random_byte_generator);

        RunningGame {
            emulator: initialized_emulator,
        }
    }
}

#[wasm_bindgen]
pub struct RunningGame {
    emulator: InitializedEmulator<
        Chip8OpCodesProcessor,
        Chip8Gpu,
        BrowserKeyboard,
        BrowserDisplay,
        RandRandomByteGenerator,
    >,
}

#[wasm_bindgen]
impl RunningGame {
    pub fn run_cycle(&mut self) -> bool {
        match self.emulator.run_cycle() {
            Ok(()) => true,
            _ => false,
        }
    }

    pub fn get_pressed_key_ptr(&self) -> *const u8 {
        self.emulator.get_keyboard().get_pressed_key_ptr()
    }
}

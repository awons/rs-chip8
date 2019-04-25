mod implementation;

use chip8::Emulator;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut buffer = Vec::with_capacity(0x1000 - 0x200);
    let mut rom = File::open(&args[1]).unwrap();
    rom.read_to_end(&mut buffer).unwrap();

    let emulator = Emulator::new();
    let keyboard = crate::implementation::keyboard::ConsoleKeyboard::new();
    let display = crate::implementation::display::ConsoleDisplay::new();
    let random_byte_generator =
        crate::implementation::random_byte_generator::RandRandomByteGenerator {};
    let mut initialized_emulator =
        emulator.initialize(&buffer, keyboard, display, random_byte_generator);
    initialized_emulator.run();
}

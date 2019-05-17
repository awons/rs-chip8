mod implementation;

use chip8::Emulator;
use implementation::display::ConsoleDisplay;
use implementation::keyboard::ConsoleKeyboard;
use implementation::random_byte_generator::RandRandomByteGenerator;
use std::env;
use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut buffer = Vec::with_capacity(0x1000 - 0x200);
    let mut rom = File::open(&args[1]).unwrap();
    rom.read_to_end(&mut buffer).unwrap();

    let emulator = Emulator::new();
    let keyboard = ConsoleKeyboard::new();
    let display = ConsoleDisplay::new();
    let random_byte_generator = RandRandomByteGenerator {};
    let mut initialized_emulator =
        emulator.initialize(&buffer, keyboard, display, random_byte_generator);

    while let Ok(()) = initialized_emulator.run_cycle() {
        sleep(Duration::from_millis(2));
    }
}

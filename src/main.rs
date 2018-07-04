mod emulator;
use emulator::keyboard::*;

fn main(){
    let mut keyboard = Keyboard::new();

    loop {
        println!("presseed key is: {}", keyboard.wait_for_key_press());
    }
}
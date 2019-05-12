use chip8::keyboard::{Key, Keyboard};

pub struct BrowserKeyboard {}

impl BrowserKeyboard {
    pub fn new() -> BrowserKeyboard {
        BrowserKeyboard {}
    }
}

impl Keyboard for BrowserKeyboard {
    fn wait_for_key_press(&mut self) -> Key {
        Key::Key0
    }

    fn get_pressed_key(&mut self) -> Option<Key> {
        None
    }
}

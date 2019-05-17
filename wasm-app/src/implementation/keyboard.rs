use chip8::keyboard::{Key, Keyboard};

pub struct BrowserKeyboard {
    pressed_key: [u8; 1],
}

impl BrowserKeyboard {
    pub fn new() -> BrowserKeyboard {
        BrowserKeyboard { pressed_key: [0] }
    }

    pub fn get_pressed_key_ptr(&self) -> *const u8 {
        self.pressed_key.as_ptr()
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

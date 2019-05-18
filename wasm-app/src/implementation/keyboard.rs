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

    fn read_key(&self) -> Option<Key> {
        match self.pressed_key[0] {
            49 => Some(Key::Key1),
            50 => Some(Key::Key2),
            51 => Some(Key::Key3),
            52 => Some(Key::KeyC),
            81 => Some(Key::Key4),
            87 => Some(Key::Key5),
            69 => Some(Key::Key6),
            82 => Some(Key::KeyD),
            65 => Some(Key::Key7),
            83 => Some(Key::Key8),
            68 => Some(Key::Key9),
            70 => Some(Key::KeyE),
            90 => Some(Key::KeyA),
            88 => Some(Key::Key0),
            67 => Some(Key::KeyB),
            86 => Some(Key::KeyF),
            27 => Some(Key::KeyESC),
            _ => None,
        }
    }
}

impl Keyboard for BrowserKeyboard {
    fn wait_for_key_press(&mut self) -> Key {
        loop {
            if let Some(key) = self.read_key() {
                return key;
            }
        }
    }

    fn get_pressed_key(&mut self) -> Option<Key> {
        self.read_key()
    }
}

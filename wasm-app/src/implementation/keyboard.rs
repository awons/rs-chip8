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
            113 => Some(Key::Key4),
            119 => Some(Key::Key5),
            101 => Some(Key::Key6),
            114 => Some(Key::KeyD),
            97 => Some(Key::Key7),
            115 => Some(Key::Key8),
            100 => Some(Key::Key9),
            102 => Some(Key::KeyE),
            122 => Some(Key::KeyA),
            120 => Some(Key::Key0),
            99 => Some(Key::KeyB),
            118 => Some(Key::KeyF),
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

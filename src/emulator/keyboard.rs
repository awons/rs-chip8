pub struct Keyboard {
    last_key: Option<u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            last_key: None
        }
    }
}

pub trait TKeyboard {
    fn wait_for_key_press(&mut self) -> u8;
    fn get_pressed_key(&mut self) -> Option<u8>;
}

impl TKeyboard for Keyboard {
    fn wait_for_key_press(&mut self) -> u8 {
        1
    }

    fn get_pressed_key(&mut self) -> Option<u8> {
        None
    }
}
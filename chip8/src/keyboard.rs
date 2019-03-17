pub trait Keyboard {
    fn wait_for_key_press(&mut self) -> Key;
    fn get_pressed_key(&mut self) -> Option<Key>;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Key {
    Key0 = 0x0,
    Key1 = 0x1,
    Key2 = 0x2,
    Key3 = 0x3,
    Key4 = 0x4,
    Key5 = 0x5,
    Key6 = 0x6,
    Key7 = 0x7,
    Key8 = 0x8,
    Key9 = 0x9,
    KeyA = 0xa,
    KeyB = 0xb,
    KeyC = 0xc,
    KeyD = 0xd,
    KeyE = 0xe,
    KeyF = 0xf,
    KeyESC = 0xff,
}

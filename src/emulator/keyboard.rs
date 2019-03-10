use std::cell::RefCell;
use std::io::Read;
use termion::{async_stdin, AsyncReader};

pub trait Keyboard {
    fn wait_for_key_press(&mut self) -> Key;
    fn get_pressed_key(&mut self) -> Option<Key>;
}

pub struct ConsoleKeyboard {
    async_reader: RefCell<AsyncReader>,
    bytes_buffer: RefCell<Vec<u8>>,
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

impl ConsoleKeyboard {
    pub fn new() -> Self {
        ConsoleKeyboard {
            async_reader: RefCell::new(async_stdin()),
            bytes_buffer: RefCell::new(Vec::new()),
        }
    }

    fn read_key(&self) -> Option<Key> {
        self.async_reader
            .borrow_mut()
            .read_to_end(&mut self.bytes_buffer.borrow_mut())
            .unwrap();
        let mut buffer = self.bytes_buffer.borrow_mut();
        let bytes = buffer.drain(..).collect::<Vec<u8>>();

        if let Some(byte) = bytes.last() {
            return self.match_byte(byte.clone());
        }

        None
    }

    fn read_key_wait(&self) -> Key {
        loop {
            if let Some(key) = self.read_key() {
                return key;
            }
        }
    }

    fn match_byte(&self, key: u8) -> Option<Key> {
        match key {
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

impl Keyboard for ConsoleKeyboard {
    fn wait_for_key_press(&mut self) -> Key {
        self.read_key_wait()
    }

    fn get_pressed_key(&mut self) -> Option<Key> {
        self.read_key()
    }
}

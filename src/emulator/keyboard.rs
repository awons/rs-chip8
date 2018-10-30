use termion::{async_stdin, AsyncReader};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::event::Key as TermKey;
use std::io::{Read, Stdout, stdin, stdout};
use std::cell::RefCell;

pub struct Keyboard {
    _raw_terminal: RawTerminal<Stdout>,
    async_reader: RefCell<AsyncReader>,
    bytes_buffer: RefCell<Vec<u8>>
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Key {
    Key0 = 48,
    Key1 = 49,
    Key2 = 50,
    Key3 = 51,
    Key4 = 52,
    Key5 = 53,
    Key6 = 54,
    Key7 = 55,
    Key8 = 56,
    Key9 = 57,
    KeyA = 97,
    KeyB = 98,
    KeyC = 99,
    KeyD = 100,
    KeyE = 101,
    KeyF = 102,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            _raw_terminal: stdout().into_raw_mode().unwrap(),
            async_reader: RefCell::new(async_stdin()),
            bytes_buffer: RefCell::new(Vec::new())
        }
    }

    fn read_key(&self) -> Option<Key> {
        self.async_reader.borrow_mut().read_to_end(&mut self.bytes_buffer.borrow_mut()).unwrap();
        let mut buffer = self.bytes_buffer.borrow_mut();
        let bytes = buffer.drain(..).collect::<Vec<u8>>();

        if let Some(byte) = bytes.last() {
            self.match_byte(byte.clone());
        }

        None
    }

    fn read_key_wait(&self) -> Key {
        for key in stdin().keys() {
            if let Some(k) = self.match_key(key.unwrap()) {
                return k
            }
            continue;
        }

        unreachable!()
    }

    fn match_byte(&self, key: u8) -> Option<Key> {
        match key {
            48 => Some(Key::Key0),
            49 => Some(Key::Key1),
            50 => Some(Key::Key2),
            51 => Some(Key::Key3),
            52 => Some(Key::Key4),
            53 => Some(Key::Key5),
            54 => Some(Key::Key6),
            55 => Some(Key::Key7),
            56 => Some(Key::Key8),
            57 => Some(Key::Key9),
            97 => Some(Key::KeyA),
            98 => Some(Key::KeyB),
            99 => Some(Key::KeyC),
            100 => Some(Key::KeyD),
            101 => Some(Key::KeyE),
            102 => Some(Key::KeyF),
            27 => {
                std::process::exit(0);
            },
            _ => None,
        }
    }

    fn match_key(&self, key: TermKey) -> Option<Key> {
        match key {
            TermKey::Char('0') => Some(Key::Key0),
            TermKey::Char('1') => Some(Key::Key1),
            TermKey::Char('2') => Some(Key::Key2),
            TermKey::Char('3') => Some(Key::Key3),
            TermKey::Char('4') => Some(Key::Key4),
            TermKey::Char('5') => Some(Key::Key5),
            TermKey::Char('6') => Some(Key::Key6),
            TermKey::Char('7') => Some(Key::Key7),
            TermKey::Char('8') => Some(Key::Key8),
            TermKey::Char('9') => Some(Key::Key9),
            TermKey::Char('a') => Some(Key::KeyA),
            TermKey::Char('b') => Some(Key::KeyB),
            TermKey::Char('c') => Some(Key::KeyC),
            TermKey::Char('d') => Some(Key::KeyD),
            TermKey::Char('e') => Some(Key::KeyE),
            TermKey::Char('f') => Some(Key::KeyF),
            TermKey::Esc => {
                std::process::exit(0);
            },
            _ => None,
        }
    }
}

pub trait TKeyboard {
    fn wait_for_key_press(&mut self) -> Key;
    fn get_pressed_key(&mut self) -> Option<Key>;
}

impl TKeyboard for Keyboard {
    fn wait_for_key_press(&mut self) -> Key {
        self.read_key_wait()
    }

    fn get_pressed_key(&mut self) -> Option<Key> {
        self.read_key()
    }
}
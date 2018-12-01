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
            return self.match_byte(byte.clone());
        }

        None
    }

    fn read_key_wait(&self) -> Key {
        for key in stdin().keys() {
            if let Some(k) = self.match_key(key.unwrap()) {
                return k
            }
        }

        unreachable!()
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

    fn match_key(&self, key: TermKey) -> Option<Key> {
        match key {
            TermKey::Char('1') => Some(Key::Key1),
            TermKey::Char('2') => Some(Key::Key2),
            TermKey::Char('3') => Some(Key::Key3),
            TermKey::Char('4') => Some(Key::KeyC),
            TermKey::Char('q') => Some(Key::Key4),
            TermKey::Char('w') => Some(Key::Key5),
            TermKey::Char('e') => Some(Key::Key6),
            TermKey::Char('r') => Some(Key::KeyD),
            TermKey::Char('a') => Some(Key::Key7),
            TermKey::Char('s') => Some(Key::Key8),
            TermKey::Char('d') => Some(Key::Key9),
            TermKey::Char('f') => Some(Key::KeyD),
            TermKey::Char('z') => Some(Key::KeyA),
            TermKey::Char('x') => Some(Key::Key0),
            TermKey::Char('c') => Some(Key::KeyB),
            TermKey::Char('v') => Some(Key::KeyF),
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
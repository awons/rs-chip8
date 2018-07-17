extern crate mio;
extern crate termios;
extern crate libc;

use termios::{Termios, TCSANOW, ICANON, ECHO, tcsetattr};
use mio::*;
use mio::unix::EventedFd;

use libc::{c_void, read};

pub struct Keyboard {
    poll: Poll,
    events: Events,
    key_buffer: [u8; 1],
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
        let mut termios = Termios::from_fd(0).unwrap().clone();
        termios.c_lflag &= !(ICANON | ECHO);
        tcsetattr(0, TCSANOW, &mut termios).unwrap();

        let poll = Poll::new().unwrap();
        let ev_fd = EventedFd(&0);
    
        poll.register(&ev_fd, Token(0), Ready::readable(), PollOpt::edge()).unwrap();

        Keyboard {
            poll,
            events: Events::with_capacity(1),
            key_buffer: [0; 1],
        }
    }

    fn read_key(key_buffer: &mut [u8; 1]) -> Option<Key> {
        unsafe {
            read(0, key_buffer.as_mut_ptr() as *mut c_void, 1);
        }
        
        match key_buffer[0] {
            0 => None,
            key_code => {
                match key_code {
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
                    _ => None,
                }
            },
        }
    }

    fn poll_key_events(&mut self) -> Option<Key> {
        let mut key: Option<Key> = None;
        self.poll.poll(&mut self.events, None).unwrap();
        for event in self.events.iter() {
            if let Token(0) = event.token() {
                key = Keyboard::read_key(&mut self.key_buffer);
            }
        }

        key
    }
}

pub trait TKeyboard {
    fn wait_for_key_press(&mut self) -> Key;
    fn get_pressed_key(&mut self) -> Option<Key>;
}

impl TKeyboard for Keyboard {
    fn wait_for_key_press(&mut self) -> Key {
        loop {
            if let Some(key) = self.poll_key_events() {
                return key;    
            }
        }
    }

    fn get_pressed_key(&mut self) -> Option<Key> {
        self.poll_key_events()
    }
}
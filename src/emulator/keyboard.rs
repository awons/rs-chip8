extern crate ncurses;

use self::ncurses::*;

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
        initscr();
        noecho();
        let mut key: i32 = -1;
        let mut found = false;

        while !found {
            key = getch();
            if (key >= 48 && key <= 57) || (key >= 97 && key <= 102) {
                found = true;
            }
        }

        key as u8
    }

    fn get_pressed_key(&mut self) -> Option<u8> {
        None
    }
}
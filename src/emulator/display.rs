use termion::screen::*;
use termion::raw::{IntoRawMode, RawTerminal};
use emulator::memory::{Memory};
use std::io::{Write, Stdout, stdout};
use std::{ops};
use std::cell::RefCell;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const SPRITE_WIDTH: u8 = 8;

struct DisplayMemory {
    memory: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT]
}

impl DisplayMemory {
    fn new() -> Self {
        DisplayMemory {
            memory: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT]
        }
    }

    fn clear(&mut self) {
        for pixel in self.memory.iter_mut() {
            *pixel = 0;
        }
    }
}

impl ops::Index<usize> for DisplayMemory {
    type Output = [u8];

    fn index(&self, row: usize) -> &[u8] {
        let start = row * DISPLAY_WIDTH;
        &self.memory[start .. start + DISPLAY_WIDTH]
    }
}

impl ops::IndexMut<usize> for DisplayMemory {
    fn index_mut(&mut self, row: usize) -> &mut [u8] {
        let start = row * DISPLAY_WIDTH;
        &mut self.memory[start .. start + DISPLAY_WIDTH]
    }
}

pub struct Display {
    memory: DisplayMemory,
    raw_terminal: RefCell<AlternateScreen<RawTerminal<Stdout>>>
}

impl Display {
    pub fn new() -> Self {
        let mut terminal = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        write!(terminal,
               "{}{}",
               termion::clear::All,
               termion::cursor::Hide)
            .unwrap();
        terminal.flush().unwrap();

        Self {
            memory: DisplayMemory::new(),
            raw_terminal: RefCell::new(terminal),
        }
    }

    fn draw_on_canvas(&self, x: u8, y: u8, pixel: u8) {
            let character;
            if pixel == 0 {
                character = " ";
            } else {
                character = "0";
            }

            write!(self.raw_terminal.borrow_mut(),
                   "{}{}",
                   termion::cursor::Goto((x + 1) as u16, (y + 1) as u16),
                   character)
                .unwrap();
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        let mut terminal = self.raw_terminal.borrow_mut();

        write!(terminal,
               "{}{}",
               termion::clear::All,
               termion::cursor::Show)
            .unwrap();
        terminal.flush().unwrap();
    }
}

pub trait TDisplay {
    fn draw_sprite(&mut self, start_x: u8, start_y: u8, rows: u8, address_register: &u16, memory: &Memory) -> bool;
    fn clear(&mut self);
}

impl TDisplay for Display {
    fn clear(&mut self) {
        self.memory.clear();
        let mut terminal = self.raw_terminal.borrow_mut();
        write!(terminal,
               "{}",
               termion::clear::All)
            .unwrap();
        terminal.flush().unwrap();
    }

    fn draw_sprite(&mut self, start_x: u8, start_y: u8, rows: u8, address_register: &u16, memory: &Memory) -> bool {
        let mut is_flipped = false;
        let mut i = 0;

        for sprite_position_y in 0..rows {
            let sprite_new_row = memory.read(*address_register + i);
            let mask: u8 = 0b1000_0000;

            let mut display_y;
            if (start_y + sprite_position_y + 1) as usize > DISPLAY_HEIGHT {
                display_y = start_y + sprite_position_y - DISPLAY_HEIGHT as u8;
            } else {
                display_y = start_y + sprite_position_y;
            }

            for sprite_position_x in 0..SPRITE_WIDTH {
                let mut display_x;
                if (start_x + sprite_position_x + 1) as usize > DISPLAY_WIDTH {
                    display_x = sprite_position_x;
                } else {
                    display_x = start_x + sprite_position_x;
                }
                let current_mask = mask.rotate_right(sprite_position_x as u32);

                let old_pixel = self.memory[display_y as usize][display_x as usize];
                let new_pixel = (sprite_new_row & current_mask).rotate_left(sprite_position_x as u32 + 1);
                let xor_pixel = old_pixel ^ new_pixel;
                self.memory[display_y as usize][display_x as usize] = xor_pixel;
                if old_pixel & new_pixel == 1 {
                    is_flipped = true;
                }

                self.draw_on_canvas(display_x, display_y, xor_pixel);
            }

            i += 1;
        }

        is_flipped
    }
}

#[cfg(test)]
mod test_display {
    use super::{Display, TDisplay};
    use emulator::memory::{Memory};

    impl Display {
        fn get_pixel(&self, y: u8, x: u8) -> u8 {
            self.memory[y as usize][x as usize]
        }
    }

    #[test]
    fn test_draw_sprite_not_flipped() {
        let mut memory = Memory::new();
        let address_register = 0x100;
        for address in address_register..0x110 {
            memory.write(address, 1);
        }

        let mut display = Display::new();
        let is_flipped = display.draw_sprite(0, 0, 3, &address_register, &memory);
        assert!(!is_flipped);
    }

    #[test]
    fn test_draw_sprite_flipped() {
        let mut memory = Memory::new();
        let address_register = 0x100;
        for address in address_register..0x110 {
            memory.write(address, 1);
        }

        let mut display = Display::new();
        display.draw_sprite(0, 0, 3, &address_register, &memory);
        let is_flipped = display.draw_sprite(0, 0, 3, &address_register, &memory);

        assert!(is_flipped);
    }

    #[test]
    fn test_clean_display() {
        let mut memory = Memory::new();
        let address_register = 0x100;
        for address in address_register..0x10f {
            memory.write(address, 1);
        }

        let mut display = Display::new();
        display.draw_sprite(0, 0, 3, &address_register, &memory);
        display.clear();

        for y in 0..2 {
            for x in 0..8 {
                assert_eq!(display.get_pixel(x, y), 0);
            }
        }
    }
}

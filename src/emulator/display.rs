extern crate pancurses;

use emulator::memory::{Memory};
use self::pancurses::{Window, initscr, endwin, curs_set};
use std::{ops};

use std::{thread, time};

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
    window: Window
}

impl Display {
    pub fn new() -> Self {

        let window = initscr();
        curs_set(0);

        Self {
            memory: DisplayMemory::new(),
            window,
        }
    }

    fn draw_on_canvas(&self, x: u8, y: u8, pixel: u8) {
            self.window.mv(y as i32, x as i32);

            if pixel == 0 {
                self.window.printw(" ");
            } else {
                self.window.printw("O");
            }
            //thread::sleep(time::Duration::from_millis(100));
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        endwin();
    }
}

pub trait TDisplay {
    fn draw_sprite(&mut self, x: u8, y: u8, rows: u8, address_register: &u16, memory: &Memory) -> bool;
    fn clear(&mut self);
}

impl TDisplay for Display {
    fn clear(&mut self) {
        self.memory.clear();
        self.window.clear();
    }

    fn draw_sprite(&mut self, x: u8, y: u8, rows: u8, address_register: &u16, memory: &Memory) -> bool {
        let mut is_flipped = false;
        let mut i = 0;

        for current_y in y..rows+y {
            let new_row = memory.read(*address_register + i);
            let old_row = self.memory[current_y as usize][x as usize];
            self.memory[current_y as usize][x as usize] = new_row;

            let xor_row = old_row ^ new_row;

            if old_row & new_row != 0 {
                is_flipped = true;
            }

            let mask: u8 = 0b1000_0000;
            for bit_position in 0..SPRITE_WIDTH {
                let current_mask = mask.rotate_right(bit_position as u32);
                let bit = (xor_row & current_mask).rotate_left(bit_position as u32 + 1);
                let mut current_x;
                if (x + bit_position) as usize > DISPLAY_WIDTH {
                    current_x = bit_position;
                } else {
                    current_x = x + bit_position;
                }

                self.draw_on_canvas(current_x, current_y, bit);
            }

            i += 1;
        }

        self.window.refresh();

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
    fn test_draw_sprite_flipped() {
        let mut memory = Memory::new();
        let address_register = 0x100;
        for address in address_register..0x110 {
            memory.write(address, 1);
        }

        let mut display = Display::new();
        let is_flipped = display.draw_sprite(0, 0, 3, &address_register, &memory);
        assert!(is_flipped);
    }

    #[test]
    fn test_draw_sprite_not_flipped() {
        let mut memory = Memory::new();
        let address_register = 0x100;
        for address in address_register..0x110 {
            memory.write(address, 1);
        }

        let mut display = Display::new();
        display.draw_sprite(0, 0, 3, &address_register, &memory);
        let is_flipped = display.draw_sprite(0, 0, 3, &address_register, &memory);

        assert!(!is_flipped);
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

extern crate pancurses;

use emulator::memory::{Memory};
use self::pancurses::{Window, initscr, endwin};
use std::{cmp, ops};

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
    window: Window,
    pixel_width: u8,
    pixel_height: u8
}

impl Display {
    pub fn new() -> Self {

        let window = initscr();
        let (pixel_width, pixel_height) = Display::calculate_pixel_width_height(&window);

        Self {
            memory: DisplayMemory::new(),
            window,
            pixel_width,
            pixel_height,
        }
    }

    fn calculate_pixel_width_height(window: &Window) -> (u8, u8) {
        let (real_width_px, real_height_px) = window.get_beg_yx();
        let shorter_side = cmp::min(real_width_px, real_height_px);

        let pixel_width = ((shorter_side / DISPLAY_WIDTH as i32) as f64).floor();
        let pixel_height = ((shorter_side / DISPLAY_HEIGHT as i32) as f64).floor();

        (pixel_width as u8, pixel_height as u8)
    }

    fn draw_on_canvas(&self, x: u8, y: u8, data: u8) {
        let (window_x, window_y) = self.calculate_window_x_y(x, y);
        self.window.mv(window_x as i32, window_y as i32);
        if data == 0 {
            self.window.addch(' ');
        } else {
            self.window.addch(0x25A0);
        }
    }

    fn calculate_window_x_y(&self, x: u8, y: u8) -> (usize, usize) {
        ((x * self.pixel_width as u8) as usize, (y * self.pixel_height) as usize)
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
        let mut data;
        for row in y..rows {
            for column in x..SPRITE_WIDTH {
                data = match memory.read(*address_register + i) {
                    0 => 0,
                    _ => 1,
                };
                if self.memory[row as usize][column as usize] != data {
                    is_flipped = true;
                }
                self.memory[row as usize][column as usize] = data;
                self.draw_on_canvas(column, row, data);
                i += 1;
            }
        }
        self.window.refresh();

        is_flipped
    }
}

//#[cfg(test)]
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

        for y in 0..2 {
            for x in 0..8 {
                assert_eq!(display.get_pixel(y, x), 1);
            }
        }
        for x in 0..8 {
            assert_eq!(display.get_pixel(2, x), 0);
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

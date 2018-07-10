extern crate pancurses;

use emulator::memory::{Memory};
use self::pancurses::{Window, initscr, endwin};
use std::cmp;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const SPRITE_WIDTH: u8 = 8;

pub struct Display {
    memory: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    window: Window,
    pixel_width: u8,
    pixel_height: u8
}

impl Display {
    pub fn new() -> Self {

        let window = initscr();
        let (pixel_width, pixel_height) = Display::calculate_pixel_width_height(&window);

        Self {
            memory: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
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
        self.window.addch(0x25A0);
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
        self.memory = [0; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.window.clear();
    }

    fn draw_sprite(&mut self, x: u8, y: u8, rows: u8, address_register: &u16, memory: &Memory) -> bool {
        let mut is_flipped = false;

        let mut i = 0;
        let mut data;
        for position_x in x..=SPRITE_WIDTH {
            for position_y in y..=rows {
                data = memory.read(*address_register + i);
                if self.memory[x as usize * y as usize] != data {
                    is_flipped = true;
                }
                self.memory[x as usize * y as usize] = data;
                println!("{}, {}, {}", position_x, position_y, data);
                self.draw_on_canvas(position_x, position_y, self.memory[x as usize * y as usize]);
                i += 1;
            }
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
        fn get_pixel(&self, x: u8, y: u8) -> u8 {
            self.memory[x as usize * y as usize]
        }
    }

    #[test]
    fn test_draw_sprite() {
        let mut memory = Memory::new();
        let address_register = 0x100;
        for address in address_register..0x10f {
            memory.write(address, 1);
        }

        let mut display = Display::new();
        let is_flipped = display.draw_sprite(1, 1, 8, &address_register, &memory);
        assert!(is_flipped);

        for x in 1..=2 {
            for y in 1..=2 {
                assert_eq!(display.get_pixel(x, y), 1);
            }
        }
    }

    #[test]
    fn test_clean_display() {
        let mut memory = Memory::new();
        let address_register = 0x100;
        for address in address_register..0x10f {
            memory.write(address, 1);
        }

        let mut display = Display::new();
        let is_flipped = display.draw_sprite(1, 1, 8, &address_register, &memory);
        display.clear();

        for x in 1..=8 {
            for y in 1..=8 {
                assert_eq!(display.get_pixel(x, y), 0);
            }
        }
    }
}
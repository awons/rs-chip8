use crate::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::memory::Memory;
use std::ops;

const DISPLAY_MAX_X: u8 = DISPLAY_WIDTH as u8 - 1;
const DISPLAY_MAX_Y: u8 = DISPLAY_HEIGHT as u8 - 1;
const SPRITE_WIDTH: u8 = 8;

pub struct GraphicMemory {
    memory: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl GraphicMemory {
    pub fn new() -> Self {
        GraphicMemory {
            memory: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }

    fn clear(&mut self) {
        for pixel in self.memory.iter_mut() {
            *pixel = 0;
        }
    }
}

impl ops::Index<usize> for GraphicMemory {
    type Output = [u8];

    fn index(&self, row: usize) -> &[u8] {
        let start = row * DISPLAY_WIDTH;
        &self.memory[start..start + DISPLAY_WIDTH]
    }
}

impl ops::IndexMut<usize> for GraphicMemory {
    fn index_mut(&mut self, row: usize) -> &mut [u8] {
        let start = row * DISPLAY_WIDTH;
        &mut self.memory[start..start + DISPLAY_WIDTH]
    }
}

pub struct Chip8Gpu {
    memory: GraphicMemory,
}

impl Chip8Gpu {
    pub fn new() -> Self {
        Chip8Gpu {
            memory: GraphicMemory::new(),
        }
    }
}

pub trait Gpu {
    fn draw_sprite(
        &mut self,
        start_x: u8,
        start_y: u8,
        rows: u8,
        address_register: u16,
        memory: &Memory,
    ) -> bool;
    fn clear(&mut self);
    fn get_memory(&self) -> &GraphicMemory;
}

impl Gpu for Chip8Gpu {
    fn clear(&mut self) {
        self.memory.clear();
    }

    fn draw_sprite(
        &mut self,
        start_x: u8,
        start_y: u8,
        rows: u8,
        address_register: u16,
        memory: &Memory,
    ) -> bool {
        let mut is_flipped = false;

        let mut display_y = if start_y > DISPLAY_MAX_Y as u8 {
            start_y % (DISPLAY_HEIGHT as u8)
        } else {
            start_y
        };

        for row in 0..rows {
            let sprite_new_row = memory.read(address_register + u16::from(row));
            let mask: u8 = 0b1000_0000;

            if display_y > DISPLAY_MAX_Y {
                continue;
            }

            let mut display_x;
            if start_x > DISPLAY_MAX_X as u8 {
                display_x = start_x % (DISPLAY_WIDTH as u8)
            } else {
                display_x = start_x;
            }
            for sprite_position_x in 0..SPRITE_WIDTH {
                if display_x > DISPLAY_MAX_X {
                    continue;
                }

                let current_mask = mask.rotate_right(u32::from(sprite_position_x));

                let old_pixel = self.memory[display_y as usize][display_x as usize];
                let new_pixel =
                    (sprite_new_row & current_mask).rotate_left(u32::from(sprite_position_x) + 1);
                let xor_pixel = old_pixel ^ new_pixel;
                self.memory[display_y as usize][display_x as usize] = xor_pixel;
                if old_pixel & new_pixel == 1 {
                    is_flipped = true;
                }

                display_x += 1;
            }
            display_y += 1;
        }

        is_flipped
    }

    fn get_memory(&self) -> &GraphicMemory {
        &self.memory
    }
}

#[cfg(test)]
mod test_display {
    use super::{Chip8Gpu, Gpu};
    use crate::memory::Memory;

    impl Chip8Gpu {
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

        let mut gpu = Chip8Gpu::new();
        let is_flipped = gpu.draw_sprite(0, 0, 3, address_register, &memory);
        assert!(!is_flipped);
    }

    #[test]
    fn test_draw_sprite_flipped() {
        let mut memory = Memory::new();
        let address_register = 0x100;
        for address in address_register..0x110 {
            memory.write(address, 1);
        }

        let mut gpu = Chip8Gpu::new();
        gpu.draw_sprite(0, 0, 3, address_register, &memory);
        let is_flipped = gpu.draw_sprite(0, 0, 3, address_register, &memory);

        assert!(is_flipped);
    }

    #[test]
    fn test_clean_display() {
        let mut memory = Memory::new();
        let address_register = 0x100;
        for address in address_register..0x10f {
            memory.write(address, 1);
        }

        let mut gpu = Chip8Gpu::new();
        gpu.draw_sprite(0, 0, 3, address_register, &memory);
        gpu.clear();

        for y in 0..2 {
            for x in 0..8 {
                assert_eq!(gpu.get_pixel(x, y), 0);
            }
        }
    }
}

const GRAPHIC_MEMORY_SIZE: usize = 0x800;

pub struct Display {
    memory: [u8; GRAPHIC_MEMORY_SIZE],
}

impl Display {
    pub fn new() -> Self {
        Self {
            memory: [0; GRAPHIC_MEMORY_SIZE],
        }
    }
}

pub trait TDisplay {
    fn draw_sprite(x: u8, y: u8, row: u8) -> bool;
    fn clear();
}
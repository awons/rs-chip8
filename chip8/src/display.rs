use std::ops;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub trait GraphicDisplay {
    fn draw<M>(&mut self, memory: &M)
    where
        M: ops::Index<usize, Output = [u8]>;
}

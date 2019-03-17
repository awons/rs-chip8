use chip8::display::*;
use std::io::{stdout, Stdout, Write};
use std::ops;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct ConsoleDisplay {
    terminal: RawTerminal<Stdout>,
}

impl ConsoleDisplay {
    pub fn new() -> Self {
        let mut terminal = stdout().into_raw_mode().unwrap();
        write!(terminal, "{}{}", termion::cursor::Hide, termion::clear::All).unwrap();
        terminal.flush().unwrap();

        ConsoleDisplay { terminal }
    }
}

impl Drop for ConsoleDisplay {
    fn drop(&mut self) {
        write!(
            self.terminal,
            "{}{}",
            termion::clear::All,
            termion::cursor::Show
        )
        .unwrap();
        self.terminal.flush().unwrap();
    }
}

impl GraphicDisplay for ConsoleDisplay {
    fn draw<M>(&mut self, memory: &M)
    where
        M: ops::Index<usize, Output = [u8]>,
    {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let character = if memory[y][x] == 1 { '*' } else { ' ' };
                write!(
                    self.terminal,
                    "{}{}",
                    termion::cursor::Goto((x + 1) as u16, (y + 1) as u16),
                    character
                )
                .unwrap();
            }
        }
        self.terminal.flush().unwrap();
    }
}

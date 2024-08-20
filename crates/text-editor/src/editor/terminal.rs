use std::io::{self, stdout, Error, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{self, Clear, ClearType},
};

#[derive(Clone, Copy)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

impl Size {
    pub fn new(height: u16, width: u16) -> Self {
        Self { height, width }
    }
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        terminal::enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position::new(0, 0))?;
        Self::execute()
    }

    pub fn terminate() -> Result<(), Error> {
        terminal::disable_raw_mode()
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        queue!(io::stdout(), MoveTo(position.x, position.y))
    }

    /// 清空屏幕
    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Print("\x1B[2J\x1B[3J\x1B[H"))
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)
    }

    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))
    }

    pub fn size() -> Result<(u16, u16), Error> {
        terminal::size()
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }
}

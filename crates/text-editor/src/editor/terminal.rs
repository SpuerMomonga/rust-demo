use std::io::{stdout, Error, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Command,
};

#[derive(Clone, Copy, Default)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

impl Size {
    pub fn new(height: usize, width: usize) -> Self {
        Self { height, width }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Position {
    pub fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }
}

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        terminal::enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::execute()
    }

    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::show_caret()?;
        terminal::disable_raw_mode()
    }

    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)
    }

    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)
    }

    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))
    }

    /// 清空屏幕
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Print("\x1B[2J\x1B[3J\x1B[H"))
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))
    }

    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_caret_to(Position::new(0, row))?;
        Self::clear_line()?;
        Self::print(line_text)
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = terminal::size()?;
        Ok(Size::new(height as usize, width as usize))
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)
    }
}

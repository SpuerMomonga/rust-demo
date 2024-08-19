use std::io::{self, stdout};

use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{Event, KeyEvent, KeyModifiers};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, terminal};

#[derive(Default)]
pub struct Editor {
    shoul_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn initialize() -> Result<(), io::Error> {
        terminal::enable_raw_mode()?;
        Self::clear_screen()
    }

    fn terminate() -> Result<(), std::io::Error> {
        terminal::disable_raw_mode()
    }

    /// 清空屏幕
    fn clear_screen() -> Result<(), io::Error> {
        execute!(stdout(), Clear(ClearType::All))
    }

    fn repl(&mut self) -> Result<(), io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.shoul_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code: Char('q'),
            modifiers: KeyModifiers::ALT,
            ..
        }) = event
        {
            self.shoul_quit = true;
        }
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        if self.shoul_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}

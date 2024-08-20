use std::io::Error;

use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{Event, KeyEvent, KeyModifiers};
use terminal::{Position, Terminal};

mod terminal;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn draw_rows(&mut self) -> Result<(), Error> {
        let height = Terminal::size()?.1;
        for current_row in 0..height {
            print!("~");
            if current_row + 1 < height {
                print!("\r\n");
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
            self.should_quit = true;
        }
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            self.draw_rows()?;
            Terminal::move_cursor_to(Position::new(0, 0))?;
        }
        Ok(())
    }
}

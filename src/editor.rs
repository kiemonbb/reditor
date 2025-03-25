use crossterm::event::KeyCode::Char;
use crossterm::event::{read, Event::Key};
use crossterm::event::{Event, KeyEvent, KeyModifiers};

use crate::terminal::Terminal;

pub struct Editor {
    is_running: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Editor { is_running: true }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.read_loop().unwrap();
        Terminal::terminate().unwrap();
    }

    fn read_loop(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh()?;
            if !self.is_running {
                Terminal::print("Bye!\r\n")?;
                Terminal::flush()?;
                break;
            }
            let event = read()?;
            self.handle_event(&event);
        }
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.is_running = false;
                }
                _ => (),
            }
        }
    }
    fn draw_tildes(&mut self) -> Result<(), std::io::Error> {
        let size = Terminal::size()?;
        for row in 0..size.height {
            Terminal::print("~")?;
            if row < size.height - 1 {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn refresh(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor(0, 0)?;
        if !self.is_running {
            Terminal::clear_screen()?;
        } else {
            self.draw_tildes()?;
            self.draw_welcome()?;
        }
        Terminal::move_cursor(0, 0)?;
        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }

    fn draw_welcome(&mut self) -> Result<(), std::io::Error> {
        let size = Terminal::size()?;
        Terminal::move_cursor((size.width / 2 - 4) as u16, (size.height / 3) as u16)?;
        Terminal::print("Welcome!")?;
        Ok(())
    }
}

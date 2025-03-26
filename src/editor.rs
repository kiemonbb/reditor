use crossterm::event::KeyCode::Char;
use crossterm::event::{read, Event::Key};
use crossterm::event::{Event, KeyEvent, KeyModifiers};

use crate::terminal::{Size, Terminal};
use crate::view::View;

pub struct Editor {
    pub is_running: bool,
    pub size:Size,
}

impl Editor {
    pub fn new() -> Result<Self,std::io::Error> {
        let size = Terminal::size()?;
        Ok(Editor { is_running: true, size})
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.read_loop().unwrap();
        Terminal::terminate().unwrap();
    }

    fn read_loop(&mut self) -> Result<(), std::io::Error> {
        loop {
            View::refresh(&self)?;
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
    
}

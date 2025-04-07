use crossterm::event::KeyCode::{self, Char};
use crossterm::event::{read, Event::Key};
use crossterm::event::{Event, KeyEvent, KeyModifiers};
use std::cmp;
use std::io::Error;


use crate::buffer::Buffer;
use crate::terminal::Position;
use crate::terminal::{Size, Terminal};
use crate::view::View;

pub struct Editor {
    pub buffer: Buffer,
    pub is_running: bool,
    pub size: Size,
    pub cursor: Position,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let size = Terminal::size()?;
        let cursor = Position::default();
        let buffer = Buffer::default();
        Ok(Editor {
            is_running: true,
            size,
            cursor,
            buffer,
            
        })
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.read_loop().unwrap();
        Terminal::terminate().unwrap();
    }

    fn read_loop(&mut self) -> Result<(), Error> {
        loop {
            View::refresh(&self)?;
            if !self.is_running {
                break;
            }
            let event = read()?;
            self.handle_event(&event)?;
        }
        Ok(())
    }
//TODO split handle_event function into smaller ones
    fn handle_event(&mut self, event: &Event) -> Result<(),Error> {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.is_running = false;
                }
                KeyCode::Down => {
                    let size = Terminal::size()?;
                    self.cursor.y = cmp::min(self.cursor.y.saturating_add(1), size.height-1);
                }
                KeyCode::Up => {
                    self.cursor.y = self.cursor.y.saturating_sub(1);
                }
                KeyCode::Right => {
                    let size = Terminal::size()?;
                    self.cursor.x = cmp::min(self.cursor.x.saturating_add(1), size.width-1);
                }
                KeyCode::Left => {
                    self.cursor.x = self.cursor.x.saturating_sub(1);
                }
                KeyCode::Home => {
                    self.cursor = Position { x: 0, y: 0 };
                }
                _ => (),
            }
        }
        Ok(())
    }
}

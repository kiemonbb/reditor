use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::{stdout, Error, Write};

#[derive(Debug)]
pub struct Terminal {}

#[derive(Debug)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl Terminal {
    //gets the terminal ready for our editor to work with
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_mode()?;
        Self::clear_screen()?;
        Self::move_cursor(&Position { x: 0, y: 0 })?;
        Ok(())
    }
    //flushes all commands that are inside queue and disables raw mode
    pub fn terminate() -> Result<(), Error> {
        Self::exit_alternate_mode()?;
        Self::flush()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size { width, height })
    }

    //---QUEUE COMMANDS---
    pub fn move_cursor(position: &Position) -> Result<(), Error> {
        stdout().queue(MoveTo(position.x, position.y))?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), Error> {
        stdout().queue(Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), Error> {
        stdout().queue(Show)?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        stdout().queue(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        stdout().queue(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn enter_alternate_mode() -> Result<(), Error> {
        stdout().queue(EnterAlternateScreen)?;
        Ok(())
    }
    pub fn exit_alternate_mode() -> Result<(), Error> {
        stdout().queue(LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn print(str: &str) -> Result<(), Error> {
        stdout().queue(Print(str))?;
        Ok(())
    }
    pub fn flush() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}

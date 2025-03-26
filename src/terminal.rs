use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::{stdout, Write};

#[derive(Debug)]
pub struct Terminal {}

#[derive(Debug)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Terminal {
    //gets the terminal ready for our editor to work with
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor(0, 0)?;
        Ok(())
    }
    //flushes all commands that are inside queue and disables raw mode
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::flush()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size { width, height })
    }

    //---QUEUE COMMANDS---
    pub fn move_cursor(x: u16, y: u16) -> Result<(), std::io::Error> {
        stdout().queue(MoveTo(x, y))?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        stdout().queue(Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), std::io::Error> {
        stdout().queue(Show)?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        stdout().queue(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        stdout().queue(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn print(str: &str) -> Result<(), std::io::Error> {
        stdout().queue(Print(str))?;
        Ok(())
    }
    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }
}

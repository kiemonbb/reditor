use crate::editor::Editor;
use crate::terminal::Position;
use crate::terminal::{Size, Terminal};
pub struct View;
use crate::buffer::Buffer;

use std::io::Error;

impl View {
    fn draw_tildes(size: &Size) -> Result<(), Error> {
        Terminal::move_cursor(&Position::default())?;
        let height = size.height;
        for row in 0..height {
            Terminal::print("~")?;
            if row < height - 1 {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    pub fn render_file(buffer: &Buffer) -> Result<(), Error> {
        Terminal::move_cursor(&Position::default())?;
        for (index, line) in buffer.content.iter().enumerate() {
            Terminal::move_cursor(&Position::new(0, index as u16))?;
            Terminal::clear_line()?;
            Terminal::print(line)?;
        }
        Ok(())
    }

    pub fn refresh(editor: &Editor) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::clear_screen()?;
        if editor.is_running {
            if editor.buffer.is_empty {
                View::draw_tildes(&editor.size)?;
                View::draw_welcome(&editor.size)?;
            } else {
                View::render_file(&editor.buffer)?;
            }
        }
        Terminal::move_cursor(&editor.cursor)?;
        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }

    fn draw_welcome(size: &Size) -> Result<(), Error> {
        Terminal::move_cursor(&Position {
            x: (size.width / 2 - 4) as u16,
            y: (size.height / 3) as u16,
        })?;
        Terminal::print("Welcome!")?;
        Ok(())
    }
}

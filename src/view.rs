use crate::terminal::{Terminal,Size};
use crate::editor::Editor;
pub struct View;

impl View {
    fn draw_tildes(size:&Size) -> Result<(), std::io::Error> {
        let height = size.height;
        for row in 0..height {
            Terminal::print("~")?;
            if row < height - 1 {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    pub fn refresh(editor:&Editor) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor(0, 0)?;
        if !editor.is_running {
            Terminal::clear_screen()?;
        } else {
            View::draw_tildes(&editor.size)?;
            View::draw_welcome(&editor.size)?;
        }
        Terminal::move_cursor(0, 0)?;
        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }

    fn draw_welcome(size:&Size) -> Result<(), std::io::Error> {
        Terminal::move_cursor((size.width / 2 - 4) as u16, (size.height / 3) as u16)?;
        Terminal::print("Welcome!")?;
        Ok(())
    }

}

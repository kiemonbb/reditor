use crate::terminal::{Terminal,Size};
use crate::editor::Editor;
use crate::terminal::Position;
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
        Terminal::move_cursor(&Position::default())?;
        Terminal::clear_screen()?;
        if editor.is_running {
            View::draw_tildes(&editor.size)?;
            View::draw_welcome(&editor.size)?;
        }
        Terminal::move_cursor(&editor.cursor)?;
        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }

    fn draw_welcome(size:&Size) -> Result<(), std::io::Error> {
        Terminal::move_cursor(&Position {x:(size.width / 2 - 4) as u16, y: (size.height / 3) as u16} )?;
        Terminal::print("Welcome!")?;
        Ok(())
    }

}

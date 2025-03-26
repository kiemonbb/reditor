#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod view;

use editor::Editor;

fn main() -> Result<(),std::io::Error> {
    let mut editor = Editor::new()?;
    editor.run();
    Ok(())
}

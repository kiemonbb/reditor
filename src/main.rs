#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;

use editor::Editor;

fn main() -> Result<(),std::io::Error> {
    let mut editor = Editor::default();
    editor.run();
    Ok(())
}

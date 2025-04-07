#![warn(clippy::all, clippy::pedantic)]
mod buffer;
mod editor;
mod terminal;
mod view;

use editor::Editor;
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut editor = Editor::new()?;
    editor.run();
    Ok(())
}

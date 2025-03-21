#![warn(clippy::all, clippy::pedantic)]
mod editor;

use anyhow::Result;
use editor::Editor;

const CLEAR_SCREEN: &str = "\x1b[2J";

fn main() -> Result<()> {
    Editor::run();
        Ok(())
}

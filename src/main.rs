use std::io::{self, Read};

use anyhow::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

const CLEAR_SCREEN: &str = "\x1b[2J";

fn main() -> Result<()> {
    enable_raw_mode().unwrap();
    for key in io::stdin().bytes() {
        let b = key.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:#03}", b);
        } else {
            println!("{0:#?}", c);
        }
        if c == 'q' {
            disable_raw_mode().unwrap();
            print!("\x1b[2J");
            return Ok(());
        }
    }
    Ok(())
}

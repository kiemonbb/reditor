use crossterm::event::KeyCode::Char;
use crossterm::{
    event::{read, Event::Key},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub struct Editor {}

impl Editor {
    pub fn run() {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                        println!("{c}");
                    }
                }
                Err(err) => println!("Error: {err}"),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
}

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::PathBuf;

//TODO switch from a line buffer to gap buffer or piece table one
pub struct Buffer {
    pub content: Vec<String>,
    pub is_empty: bool,
}

impl Buffer {
    pub fn clear_buffer(&mut self) {
        self.content.clear();
        self.is_empty = true;
    }

    //TODO handle File::open() error
    pub fn load_file(&mut self, path: PathBuf) -> Result<(), Error> {
        self.clear_buffer();
        self.is_empty = false;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            self.content.push(line?);
        }
        Ok(())
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            content: Vec::new(),
            is_empty: true,
        }
    }
}

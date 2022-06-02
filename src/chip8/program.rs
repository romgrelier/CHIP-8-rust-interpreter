use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Program {
    pub content: Vec<u8>,
}

impl From<String> for Program {
    fn from(filename: String) -> Self {
        let file = File::open(filename).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut content = Vec::new();
        buf_reader.read_to_end(&mut content).unwrap();

        Self {
            content: content
        }
    }
}

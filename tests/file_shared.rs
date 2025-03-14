use std::fs::{File, OpenOptions};
use std::io::BufRead;

pub fn read_file_line(path: String, index: u16) -> String {
    File::open(path)
        .and_then(|file| {
            let lines = std::io::BufReader::new(file).lines();
            lines.skip(index as usize).next().unwrap()
        })
        .unwrap()
}

pub fn clean_file(path: String) {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();
}

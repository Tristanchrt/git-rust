use std::fs;
use std::fs::{File};
use std::io::BufRead;
use std::thread::sleep;
use std::time::Duration;

pub fn read_file_line(path: String, index: u16) -> String {
    File::open(path)
        .and_then(|file| {
            let lines = std::io::BufReader::new(file).lines();
            lines.skip(index as usize).next().unwrap()
        })
        .unwrap()
}

pub fn clean_file(path: String) {
    match fs::remove_file(&path) {
        Ok(_) => println!("File deleted successfully"),
        Err(e) => println!("Failed to delete file: {}", e),
    }
    match File::create(&path) {
        Ok(_) => println!("File created successfully"),
        Err(e) => println!("Failed to create file: {}", e),
    }
    sleep(Duration::new(0, 500_000_000));
}

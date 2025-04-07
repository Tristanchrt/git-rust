use std::fs;
use std::io::Write;
use std::fs::{File, OpenOptions};
use std::io::BufRead;
use std::thread::sleep;
use std::time::Duration;
use git_rust::settings::load_settings;

pub fn read_file_line(path: String, index: u16) -> String {
    File::open(path)
        .and_then(|file| {
            let lines = std::io::BufReader::new(file).lines();
            lines.skip(index as usize).next().unwrap()
        })
        .unwrap()
}

pub fn read_file(path: String) -> String {
    fs::read_to_string(path).unwrap()
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
    sleep(Duration::new(0, 250_000_000));
}

pub fn clean_dir(path: String) {
    fs::remove_dir_all(path).unwrap();
}

pub fn write_in_file(path: String, content: String) {
    let mut file = OpenOptions::new()
        .append(false)
        .open(path)
        .expect("Couldn't open file");

    if let Err(e) = writeln!(file, "{}", content) {
        panic!("Couldn't save commit: {}", e);
    }
}

pub fn clean_db_test() {
    let settings = load_settings();

    clean_file(settings.db_branches);
    clean_file(settings.db_current_branch);
    clean_file(settings.db_commits);

    sleep(Duration::new(0, 250_000_000));
}
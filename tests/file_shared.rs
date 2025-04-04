use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::thread::sleep;
use std::time::Duration;
use git_rust::settings;
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

pub fn clean_db_test() {
    let settings = load_settings();

    clean_file(settings.db_branches);
    clean_file(settings.db_current_branch);
    clean_file(settings.db_commits);

    sleep(Duration::new(0, 250_000_000));
}
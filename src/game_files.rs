use std::io::{BufRead, BufReader};
use std::{fs, io};
use std::path::Path;

// Import the files as string constants
pub fn get_seed_files_list() -> Vec<String> {
    let seeds_path = "src/seeds/".to_string();
    let mut seed_files = Vec::new();
    if let Ok(entries) = fs::read_dir(seeds_path.clone()) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    seed_files.push(format!("{}{}",seeds_path, file_name.to_string()));
                }
            }
        }
    }
    seed_files
}

pub fn get_seed_file_from_index(index: u32) -> String {
    let real_index = (index % get_seed_files_list().len() as u32 )as usize;
    get_seed_files_list()[real_index].clone()
}

pub fn read_file_lines(path: &Path) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

// Import the files as string constants
const GLIDER_GUN: &str = include_str!("data/glider_gun.life");
const PULSAR: &str = include_str!("data/pulsar.life");
const SPACESHIP: &str = include_str!("data/spaceship.life");
const BRAILLE: &str = include_str!("data/braille.life");

// Define an enum to identify each file
pub enum GameFile {
    GliderGun,
    Pulsar,
    Spaceship,
    Braille
}

// Function to get the file content based on enum selection
pub fn get_game_file_content(file: GameFile) -> &'static str {
    match file {
        GameFile::GliderGun => GLIDER_GUN,
        GameFile::Pulsar => PULSAR,
        GameFile::Spaceship => SPACESHIP,
        GameFile::Braille => BRAILLE,
    }
}

// Function to get the file content based on enum selection
pub fn get_game_file_content_from_str(name: &str) -> &'static str {
    match name {
        "glider_gun" => GLIDER_GUN,
        "pulsar" => PULSAR,
        "spaceship" => SPACESHIP,
        "braille" => BRAILLE,
        _ => BRAILLE
    }
}


// // Function to get the file content based on enum selection
// pub fn get_game_file_content_from_path(path: &str) -> &'static str {
//
// }
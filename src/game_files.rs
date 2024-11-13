use std::env::consts::OS;
use crate::game_files::GameSeed::Oscillator;

// Import the files as string constants
const GLIDER_GUN: &str = include_str!("data/glider_gun.life");
const PULSAR: &str = include_str!("data/pulsar.life");
const SPACESHIP: &str = include_str!("data/spaceship.life");
const SPACESHIP_FACTORY: &str = include_str!("data/spaceship_factory.life");
const BRAILLE: &str = include_str!("data/braille.life");
const UNIT_CELL: &str = include_str!("data/unit_cell.life");
const OSCILLATOR: &str = include_str!("data/oscillator.life");

// Define an enum to identify each file
pub enum GameSeed {
    GliderGun,
    Pulsar,
    Spaceship,
    SpaceshipFactory,
    Braille,
    UnitCell,
    Oscillator,
}

impl GameSeed {
    pub(crate) fn clone(&self) -> GameSeed {
        match self {
            GameSeed::GliderGun => GameSeed::GliderGun,
            GameSeed::Pulsar => GameSeed::Pulsar,
            GameSeed::Spaceship => GameSeed::Spaceship,
            GameSeed::SpaceshipFactory => GameSeed::SpaceshipFactory,
            GameSeed::Braille => GameSeed::Braille,
            GameSeed::UnitCell => GameSeed::UnitCell,
            GameSeed::Oscillator => GameSeed::Oscillator,
        }
    }
}

// Function to get the file content based on enum selection
pub fn get_content_from_seed(file: GameSeed) -> &'static str {
    match file {
        GameSeed::GliderGun => GLIDER_GUN,
        GameSeed::Pulsar => PULSAR,
        GameSeed::Spaceship => SPACESHIP,
        GameSeed::SpaceshipFactory => SPACESHIP_FACTORY,
        GameSeed::Braille => BRAILLE,
        GameSeed::UnitCell => UNIT_CELL,
        GameSeed::Oscillator => OSCILLATOR,
    }
}

// Function to get the file content based on enum selection
pub fn get_game_file_content_from_str(name: &str) -> &'static str {
    match name {
        "glider_gun" => GLIDER_GUN,
        "pulsar" => PULSAR,
        "spaceship" => SPACESHIP,
        "spaceship_factory" => SPACESHIP_FACTORY,
        "braille" => BRAILLE,
        "unit_cell" => UNIT_CELL,
        "oscillator" => OSCILLATOR,
        _ => BRAILLE
    }
}


// // Function to get the file content based on enum selection
// pub fn get_game_file_content_from_path(path: &str) -> &'static str {
//
// }
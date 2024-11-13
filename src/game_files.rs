use std::env::consts::OS;

// Import the files as string constants
const GLIDER_GUN: &str = include_str!("seeds/glider_gun.life");
const PULSAR: &str = include_str!("seeds/pulsar.life");
const SPACESHIP: &str = include_str!("seeds/spaceship.life");
const SPACESHIP_FACTORY: &str = include_str!("seeds/spaceship_factory.life");
const BRAILLE: &str = include_str!("seeds/braille.life");
const UNIT_CELL: &str = include_str!("seeds/unit_cell.life");
const OSCILLATOR: &str = include_str!("seeds/oscillator.life");
const HERSHEL: &str = include_str!("seeds/herschel_loop.life");
const RLE28: &str = include_str!("seeds/herschel_loop.life");

// Define an enum to identify each file
pub enum GameSeed {
    GliderGun,
    Pulsar,
    Spaceship,
    SpaceshipFactory,
    Braille,
    UnitCell,
    Oscillator,
    HERSHEL,
    RLE28,
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
            GameSeed::HERSHEL => GameSeed::HERSHEL,
            GameSeed::RLE28 => GameSeed::RLE28,
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
        GameSeed::HERSHEL => HERSHEL,
        GameSeed::RLE28 => RLE28,
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
        "hershel" => HERSHEL,
        "rle28" => RLE28,
        _ => BRAILLE
    }
}

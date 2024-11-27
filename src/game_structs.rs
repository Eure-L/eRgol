use strum_macros::Display;
use crate::board::Board;
use crate::game_files::GameSeed;
use crate::kernels::Kernels;
use crate::kernels::Kernels::CpuSequential;

#[derive(Display, PartialEq, Eq)]
pub enum GameModes {
    Playing,
    MainMenu,
}

impl Clone for GameModes {
    fn clone(&self) -> GameModes {
        match self {
            GameModes::Playing => GameModes::Playing,
            GameModes::MainMenu => GameModes::MainMenu,
        }
    }
}

pub(crate) struct GameParams {
    pub(crate) iteration: u32,
    pub(crate) speed: u32,
    pub(crate) paused: bool,
    pub(crate) mode: GameModes,
    pub(crate) seed: GameSeed,
    pub(crate) kernel: Kernels,
}

impl GameParams {
    pub fn clone(&self) -> GameParams {
        GameParams {
            iteration: self.iteration,
            speed: self.speed,
            paused: self.paused,
            seed: self.seed.clone(),
            mode: self.mode.clone(),
            kernel: self.kernel.clone(),
        }
    }
}

pub const DEFAULT_GAME_PARAMS: GameParams = GameParams {
    iteration: 0,
    speed: 1,
    paused: true,                // "
    mode: GameModes::Playing,    // Doesnt matter to the rendering thread
    seed: GameSeed::Braille,     // "
    kernel: CpuSequential,       // "
};


pub struct Game {
    pub(crate) game_params: GameParams,
    pub(crate) board: Board,
}
use std::path::PathBuf;
use std::string::ToString;
use strum_macros::Display;
use crate::board::Board;
use crate::kernels::Kernels;
use crate::kernels::Kernels::CpuSequential;

#[derive(Display, PartialEq, Eq)]
pub enum GameModes {
    Playing,
    MainMenu,
}

#[derive(Display, PartialEq, Eq)]
pub enum Rendering {
    Braille
}

impl Rendering {
    pub(crate) fn clone(&self) -> Rendering {
        match self {
            Rendering::Braille => Rendering::Braille,
        }
    }
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
    pub(crate) seed: PathBuf,
    pub(crate) kernel: Kernels,
    pub(crate) rendering: Rendering,
    pub(crate) menu_scroll: u32,
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
            rendering: self.rendering.clone(),
            menu_scroll: self.menu_scroll,
        }
    }

    pub fn default() -> GameParams {
        GameParams {
            iteration: 0,
            speed: 1,
            paused: true,                // "
            mode: GameModes::Playing,    // Doesnt matter to the rendering thread
            seed: PathBuf::from("src/seeds/braille.life"),     // "
            kernel: CpuSequential,       // "
            rendering: Rendering::Braille,
            menu_scroll: 0,
        }
    }
}


pub struct Game {
    pub(crate) game_params: GameParams,
    pub(crate) board: Board,
}
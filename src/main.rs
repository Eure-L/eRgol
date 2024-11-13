mod render;
mod board;
mod kernels;
mod globals;
mod macros;
mod game_files;
mod game_loops;

use crate::board::{init_game, Board};
use crate::game_files::{GameSeed};
use crate::game_loops::{game_menu, play};
use crate::kernels::Kernels;
use crate::render::rendering_tread;
use crossterm;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use crossterm::cursor;
use std;
use std::error::Error;
use std::sync::mpsc;

enum GameModes {
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

struct GameParams {
    iteration: u32,
    speed: u32,
    paused: bool,
    mode: GameModes,
    seed: GameSeed,
    kernel: Kernels,
}

impl GameParams {
    pub(crate) fn clone(&self) -> GameParams {
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

struct Game {
    game_params: GameParams,
    board: Board,
}

fn main() -> Result<(), Box<dyn Error>> {

    // Switch to alternate terminal
    let mut stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    std::io::stdout().execute(cursor::Hide)?;

    // Rendering loop in separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = std::thread::spawn(move || unsafe {
        rendering_tread(&render_rx);
    });

    // Start game initialization
    let mut curr_board = board::empty_board();
    let mut next_board = board::empty_board();

    let mut game_params = GameParams {
        iteration: 0,
        speed: 1,
        paused: true,
        mode: GameModes::Playing,
        kernel: Kernels::CpuSequential,
        seed: GameSeed::Braille,
    };

    init_game(&mut game_params, &mut curr_board, &mut next_board);


    // Game of life loop
    'gameloop: loop {
        let status = match game_params.mode {
            GameModes::Playing => { play(&mut game_params, &mut curr_board, &mut next_board, render_tx.clone()) }
            GameModes::MainMenu => { game_menu(&mut game_params, &mut curr_board, &mut next_board, render_tx.clone()) }
        };

        match status {
            Ok(_) => {}
            Err(_) => { break 'gameloop; }
        }
    }

    // Join threads
    drop(render_tx);
    render_handle.join().unwrap();

    // Restore main terminal
    stdout.execute(LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    std::io::stdout().execute(cursor::Show)?;
    Ok(())
}


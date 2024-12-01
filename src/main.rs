mod render;
mod board;
mod kernels;
mod globals;
mod macros;
mod game_files;
mod game_loops;
mod ui;
mod game_structs;

use crate::board::{init_game};
use crate::game_loops::{game_menu, play};
use crate::render::rendering_tread;
use crossterm;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use crossterm::cursor;
use std;
use std::error::Error;
use std::sync::mpsc;
use crate::game_structs::{GameModes, GameParams};

use simplelog;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {

    simplelog::CombinedLogger::init(vec![
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            File::create("app.log").unwrap(),
        ),
    ]).unwrap();


    // Switch to alternate terminal
    let mut stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    std::io::stdout().execute(cursor::Hide)?;

    // Start rendering loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = std::thread::spawn(move || unsafe {
        rendering_tread(&render_rx);
    });

    // Game initialization
    let mut curr_board = board::empty_board();
    let mut next_board = board::empty_board();
    let mut game_params = GameParams::default();

    init_game(&mut game_params, &mut curr_board, &mut next_board);

    // Main loop -> Game of life
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


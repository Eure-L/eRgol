mod render;
mod board;
mod kernels;
mod globals;
mod macros;
mod game_files;

use std::error::Error;
use std;
use std::sync::mpsc;
use std::time::Duration;
use crossterm;
use crossterm::event::{Event, KeyCode};
use crossterm::{cursor};
use crossterm::ExecutableCommand;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crate::board::Board;
use crate::game_files::GameFile;
use crate::kernels::update_board;
use crate::render::rendering_tread;

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
    iter: u32,
    speed: u32,
    mode: GameModes,
}

impl GameParams {
    pub(crate) fn clone(&self) -> GameParams {
        GameParams{
            iter: self.iter,
            speed: self.speed,
            mode: self.mode.clone() }
    }
}

struct Game {
    game_params: GameParams,
    board: Board,
}


fn main() -> Result<(), Box<dyn Error>>{

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
    let mut game_params = GameParams{ iter: 0, speed: 1 , mode: GameModes::Playing };
    let startgame = GameFile::Spaceship;
    board::load_board_from_gamefile(startgame, &mut curr_board);

    // Game of life loop
    let mut paused = true;
    'gameloop: loop {
        while crossterm::event::poll(Duration::from_millis(7))? {
            if let Event::Key(key_event) = crossterm::event::read()? {
                match key_event.code {
                    KeyCode::Backspace => {paused = !paused}
                    KeyCode::Esc  | KeyCode::Char('q') => {
                        break 'gameloop;
                    }

                    // unit step
                    KeyCode::Char('s') => {
                        step(&mut curr_board, &mut next_board)
                    }

                    // Speed up
                    KeyCode::Char('+') => {
                        if game_params.speed < 16 {
                            game_params.speed = game_params.speed * 2
                        }
                    }

                    // Slow down
                    KeyCode::Char('-') => {
                        if game_params.speed > 1 {
                            game_params.speed = game_params.speed / 2
                        }
                    }

                    // Pause/unpause
                    KeyCode::Char('p') => {
                        paused = !paused
                    }
                    KeyCode::Char(' ') => {
                        paused = !paused;
                        break;
                    }
                    _ => {}
                }
            }
        }

        if !paused {
            for step in 0..game_params.speed {
                crate::step(&mut curr_board, &mut next_board)
            }
        }

        // Asks to the rendering thread to draw the frame :)
        let _ = render_tx.send(Game{game_params: game_params.clone(), board: curr_board.clone() });
        std::thread::sleep(Duration::from_millis(5 ));

        if !paused {
            game_params.iter += game_params.speed;
        }

    }

    // Join threads
    drop(render_tx);
    render_handle.join().unwrap();

    // Switch back to main terminal
    stdout.execute(LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    std::io::stdout().execute(cursor::Show)?;
    Ok(())
}

fn step(prev_board: &mut board::Board, next_board: &mut board::Board){
    update_board(prev_board, next_board);
    std::mem::swap(prev_board, next_board);
}
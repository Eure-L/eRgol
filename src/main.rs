mod render;
mod board;
mod kernels;
mod globals;
mod macros;

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
use crate::kernels::update_board;

struct GameParams {
    iter: u32,
    speed: u32,
}

impl GameParams {
    pub(crate) fn clone(&self) -> GameParams {
        GameParams{iter: self.iter, speed: self.speed}
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
        let mut stdout = std::io::stdout();
        let mut prev_board = board::empty_board();
        render::render_braille(&mut stdout, &prev_board, GameParams{iter: 0, speed: 1},true);
        loop{
             let curr_game : Game = match render_rx.recv() {
                Ok(rcv_game) => { rcv_game }
                Err(_) => break,
             };
            let curr_board = curr_game.board;
            let game_params = curr_game.game_params;
            // let refresh_stdout = if game_params.iter % 10 == 0 { true } else { false };
            render::render_braille(&mut stdout, &prev_board, game_params, false);
            prev_board = curr_board;
        }
    });

    let mut game_params = GameParams{ iter: 0, speed: 1 };
    let mut curr_board = board::blinker_board();
    let mut next_board = board::empty_board();

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
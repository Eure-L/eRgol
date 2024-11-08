mod render;
mod board;
mod kernels;

use std::error::Error;
use std::{io, thread};
use std::sync::mpsc;
use std::time::Duration;
use crossterm::{terminal, ExecutableCommand, event};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crate::board::{Board, empty_board, random_board, blinker_board};
use log::{info, warn};
use crate::kernels::update_board;

fn main() -> Result<(), Box<dyn Error>>{

    // Switch to alternate terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;

    // Rendering loop in separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || unsafe {
        let mut stdout = io::stdout();
        let mut prev_board = empty_board();
        // render::render(&mut stdout, &prev_board, &prev_board, true);
        render::render_braille(&mut stdout, &prev_board, &prev_board, true);
        loop{
             let curr_board = match render_rx.recv() {
                Ok(rcv_board) => { rcv_board }
                Err(_) => break,
             };
            // render::render(&mut stdout, &prev_board, &curr_board, false);
            render::render_braille(&mut stdout, &prev_board, &curr_board, false);
            prev_board = curr_board;

        }
    });

    let mut curr_board = blinker_board();
    let mut next_board = empty_board();
    let mut iter = 0;
    let mut speed: u32 = 1;
    // Game of life loop
    let mut paused = true;
    'gameloop: loop {

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
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
                        if speed < u32::MAX / 2 {
                            speed = speed * 2
                        }
                    }

                    // Slow down
                    KeyCode::Char('-') => {
                        if speed > 1 {
                            speed = speed / 2
                        }
                    }

                    // Pause/unpause
                    KeyCode::Char('p') => {
                        paused = !paused
                    }
                    KeyCode::Char(' ') => {
                        paused = !paused
                    }
                    _ => {}
                }
            }
        }

        if !paused {
            for step in 0..speed {
                crate::step(&mut curr_board, &mut next_board)
            }
        }

        // Asks to the rendering thread to draw the frame :)
        let _ = render_tx.send(curr_board.clone());
        thread::sleep(Duration::from_millis(5 ));
        iter += 1;
    }

    // Join threads
    drop(render_tx);
    render_handle.join().unwrap();

    // Switch back to main terminal
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    terminal::disable_raw_mode();
    Ok(())
}

fn step(prev_board: &mut Board, next_board: &mut Board){
    update_board(prev_board, next_board);
    std::mem::swap(prev_board, next_board);
}
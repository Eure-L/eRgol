mod render;
mod board;

use std::error::Error;
use std::{io, thread};
use std::sync::mpsc;
use std::time::Duration;
use crossterm::{terminal, ExecutableCommand, event};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crate::board::{Board, empty_board, random_board, test_board, update_board};
use log::{info, warn};

fn main() -> Result<(), Box<dyn Error>>{
    print!("Hello World");

    // Terminal part
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;

    // Rendering loop in separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || unsafe {
        let mut stdout = io::stdout();
        let mut prev_board = empty_board();
        render::render_braille(&mut stdout, &prev_board, &prev_board, true);
        loop{
             let curr_board = match render_rx.recv() {
                Ok(rcv_board) => { rcv_board }
                Err(_) => break,
             };
            render::render_braille(&mut stdout, &prev_board, &curr_board, true);
            prev_board = curr_board;

        }
    });

    let mut curr_board = test_board();
    let mut next_board = empty_board();
    let mut iter = 0;
    // Game of life loop
    'gameloop: loop {
        let mut paused = true;
        //print!("{}", iter);
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Backspace => {paused = !paused}
                    KeyCode::Esc  | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    KeyCode::Char('s') => {
                        update_board(&mut curr_board, &mut next_board);
                        std::mem::swap(&mut curr_board, &mut next_board);
                    }
                    KeyCode::Char('p') => {
                        paused = !paused
                    }
                    _ => {}
                }
                if !paused {
                    update_board(&mut curr_board, &mut next_board);
                    std::mem::swap(&mut curr_board, &mut next_board);
                }
            }
        }

        // Asks to the rendering thread to draw the frame :)
        let _ = render_tx.send(curr_board.clone());
        thread::sleep(Duration::from_millis(200 ));
        iter += 1;
    }

    // Join threads
    drop(render_tx);
    render_handle.join().unwrap();

    // Exit game
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

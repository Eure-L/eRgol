use std::error::Error;
use std::io::{Stdout, Write};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use crossterm::cursor::MoveTo;
use crossterm::{cursor, ExecutableCommand, QueueableCommand};
use crate::board::Board;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen};
use crate::{board, get, render, Game, GameModes, GameParams};
use crate::globals::{BRAILE_ALPHABET_START, BRAILLE_SIZE_X, BRAILLE_SIZE_Y, NUM_BRAILLE_BLOCS_X, NUM_BRAILLE_BLOCS_Y, };


/// Renders the grid by computing corresponding braille character for each patch of alive/dead cells
/// Uses Char arithmetic to render the correct Braille Unicode character, making it unsafe.
pub(crate) unsafe fn render_braille(stdout: &mut Stdout, prev_board: &Board, game_params: GameParams, forced: bool) {

    if forced
    {
        stdout.queue(SetForegroundColor(Color::Yellow)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Reset)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    
    const BASE: u32 = 2;
    // Braill blocks iteration
    for bloc_x in 0..get!(NUM_BRAILLE_BLOCS_X) - 1 {
        for bloc_y in 0..get!(NUM_BRAILLE_BLOCS_Y) - 1{
            let mut code = BRAILE_ALPHABET_START; // Code of corresponding unicode char for this brail code
            for ix in 0..BRAILLE_SIZE_X {
                let x = bloc_x * BRAILLE_SIZE_X + ix;

                // First 6 Braille cells pattern computing
                for iy in 0..BRAILLE_SIZE_Y - 1 {
                    let y = bloc_y * BRAILLE_SIZE_Y + iy;
                    let weight = match prev_board[x as usize][y as usize] {
                        1 => {BASE.pow((iy + ix * (BRAILLE_SIZE_Y-1)) as u32)}
                        _ => {0}
                    };
                    code = code + weight;
                }

                // Two last Braille cells
                let weight = match prev_board[x as usize][(bloc_y * (BRAILLE_SIZE_Y) + BRAILLE_SIZE_Y -1) as usize] {
                    1 => { (ix as u32 +1) * 0x40 }
                    _ => {0}
                };
                code = code + weight;
            }
            stdout.queue(MoveTo(bloc_x as u16, bloc_y as u16)).unwrap();
            // print!("{}", code)
            print!("{}", char::from_u32_unchecked(code));
        }
    }

    stdout.queue(MoveTo(0, get!(NUM_BRAILLE_BLOCS_Y) as u16)).unwrap();
    print!("Iteration:{}   \n", game_params.iter);
    stdout.queue(MoveTo(0, get!(NUM_BRAILLE_BLOCS_Y) as u16 +1)).unwrap();
    print!("Speed:{}  ", game_params.speed);
    stdout.flush().unwrap()
}


pub(crate) unsafe fn rendering_tread(render_rx: &Receiver<Game>){
    let mut stdout = std::io::stdout();
    let mut prev_board = board::empty_board();
    render_braille(&mut stdout, &prev_board, GameParams{iter: 0, speed: 1, mode: GameModes::Playing }, true);
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
}
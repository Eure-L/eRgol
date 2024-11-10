use std::io::{Stdout, Write};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crate::board::Board;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crate::{get, GameParams};
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


pub fn render(stdout: &mut Stdout, prev_board: &Board, new_board: &Board, forced: bool){

    if forced
    {
        stdout.queue(SetBackgroundColor(Color::DarkBlue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::DarkGrey)).unwrap();
    }

    for (x, col) in new_board.iter().enumerate(){
        for (y, cell) in col.iter().enumerate(){
            stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
            match *cell {
                1 => {print!("X")}
                0 => {print!(" ")}
                _ => {print!(" ")}
            }
        }
    }
    stdout.flush().unwrap()
}
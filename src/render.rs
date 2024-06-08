use std::io::{Stdout, Write};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crate::board::Board;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use eRgol::{BRAILE_ALPHABET_START, BRAILLE_SIZE_X, BRAILLE_SIZE_Y, NUM_BRAILLE_BLOCS_X, NUM_BRAILLE_BLOCS_Y};


/// Renders the grid by computing corresponding braille character for each patch of alive/dead cells
/// Uses Char arithmetic to render the correct Braille Unicode character, making it unsafe.
pub(crate) unsafe fn render_braille(stdout: &mut Stdout, prev_board: &Board, new_board: &Board, forced: bool){

    if forced
    {
        stdout.queue(SetBackgroundColor(Color::DarkBlue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::DarkGrey)).unwrap();
    }
    
    const base: u32 = 2;
    // Braill blocks iteration
    for bloc_x in 0..NUM_BRAILLE_BLOCS_X - 1 {
        for bloc_y in 0..NUM_BRAILLE_BLOCS_Y - 1{
            let mut code = BRAILE_ALPHABET_START; // Code of corresponding unicode char for this brail code


            for ix in 0..BRAILLE_SIZE_X {
                let x = bloc_x * BRAILLE_SIZE_X + ix;

                // First 6 Braille cells pattern computing
                for iy in 0..BRAILLE_SIZE_Y - 1 {
                    let y = bloc_y * BRAILLE_SIZE_Y + iy;
                    let weight = match prev_board[x][y] {
                        1 => {base.pow((iy + ix * (BRAILLE_SIZE_Y-1)) as u32)}
                        _ => {0}
                    };
                    code = code + weight;
                }

                // Two last Braille cells
                let weight = match prev_board[x][bloc_y * (BRAILLE_SIZE_Y) + BRAILLE_SIZE_Y -1] {
                    1 => { (ix as u32 +1) * 0x40 }
                    _ => {0}
                };
                code = code + weight;

            }
            stdout.queue(MoveTo(bloc_x as u16, bloc_y as u16)).unwrap();
            // print!("{}", code)
            print!("{}", char::from_u32_unchecked(code));
        }
        // Todo implement hard-code render of edge Y Blocks
    }
    // Todo implement hard-code render of edge X Blocks

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
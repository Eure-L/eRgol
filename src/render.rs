use std::io::{Stdout, Write};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crate::board::Board;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use eRgol::{BRAILE_ALPHABET_START, BRAILLE_SIZE_X, BRAILLE_SIZE_Y, NUM_BRAILLE_BLOCS_X, NUM_BRAILLE_BLOCS_Y};

/**
Renders the grid by computing corresponding braille character for each patch of alive/dead cells
**/
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
            let mut code = 0; // Code of corresponding unicode char for this brail code
            // Braille cells iteration
            for ix in 0..BRAILLE_SIZE_X {
                let x = bloc_x * BRAILLE_SIZE_X + ix;
                for iy in 0..BRAILLE_SIZE_Y {
                    let y = bloc_y * BRAILLE_SIZE_Y + iy;
                    let weight = match prev_board[x][y] {
                        1 => {base.pow((iy + ix * BRAILLE_SIZE_Y) as u32)}
                        _ => {0}
                    };
                    code = code + weight;
                }
            }
            code = code + BRAILE_ALPHABET_START;
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
            if *cell != prev_board[x][y] {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                match *cell {
                    1 => {print!("⣿")}
                    _ => {print!("⠀")}
                }
            }
        }
    }
    stdout.flush().unwrap()
}
use std::io::{Stdout, Write};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crate::board::Board;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};

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
                    1 => {print!("💩")}
                    _ => {print!("💀")}
                }
            }
        }
    }
    stdout.flush().unwrap()
}
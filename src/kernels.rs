use crate::globals::{NUM_COLS, NUM_ROWS};
use crate::board::Board;

/// Updates a given board prev_board to in the next_board
pub fn update_board(prev_board: &mut Board, next_board: &mut Board){
    // Inner board computation as the edges are off
    for ix in 1..*NUM_COLS.read().unwrap() as usize -1 {
        for iy in 1..*NUM_ROWS.read().unwrap() as usize -1{
            let total_neighbors =
                    prev_board[ix][iy-1] +
                    prev_board[ix][iy+1] +
                    prev_board[ix-1][iy] +
                    prev_board[ix+1][iy] +
                    prev_board[ix+1][iy-1] +
                    prev_board[ix-1][iy+1] +
                    prev_board[ix-1][iy-1] +
                    prev_board[ix+1][iy+1];

            if prev_board[ix][iy] == 1 {
                next_board[ix][iy] = match total_neighbors {
                    3 => {1}
                    2 => {1}
                    _ => {0}
                }
            }
            else {
                next_board[ix][iy] = match total_neighbors {
                    3 => {1}
                    _ => {0}
                }
            }
        }
    }
}

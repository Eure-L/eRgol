use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;
use eRgol::{NUM_COLS, NUM_ROWS};

pub type Board = Vec<Vec<u8>>;

/// Returns an randomly composed board
pub fn random_board() -> Board {
    let mut rng = rand::thread_rng();
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS{

            col.push(rng.gen_range(0..1));
        }
        cols.push(col)
    }
    cols
}

/// Retrns an empty Board
pub fn empty_board() -> Board {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS{
            col.push(0);
        }
        cols.push(col)
    }
    cols
}


/// Returns a Board with 3 touching cells
pub fn blinker_board() -> Board {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS{
            col.push(0);
        }
        cols.push(col)
    }
    cols[10][10] = 1;
    cols[10][11] = 1;
    cols[10][9] = 1;
    cols
}

/// Returns a Board with 3 touching cells


pub fn load_board(board_name: &str, dst_board: &mut Board){
    let file = File::open(board_name).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Clear the previous board
    for col in dst_board.iter_mut() {
        for cell in col.iter_mut() {
            *cell = 0;
        }
    }

    // Read the file line by line and updates the board
    for (y, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        for (x, cell) in line.split_whitespace().enumerate() {
            if cell == "1" {
                dst_board[x + 1][y + 1] = 1;
            }
        }
    }

}

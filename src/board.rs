use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;
use crate::game_files::{get_game_file_content, GameFile};
use crate::get;
use crate::globals::{NUM_COLS, NUM_ROWS};

pub type Board = Vec<Vec<u8>>;

/// Returns an randomly composed board
pub fn random_board() -> Board {
    let mut rng = rand::thread_rng();
    let mut cols = Vec::with_capacity(get!(NUM_COLS) as usize);
    for _ in 0..*NUM_COLS.read().unwrap()  as usize {
        let mut col = Vec::with_capacity(get!(NUM_ROWS)  as usize);
        for _ in 0..*NUM_ROWS.read().unwrap() as usize {

            col.push(rng.gen_range(0..1));
        }
        cols.push(col)
    }
    cols
}

/// Returns an empty Board
pub fn empty_board() -> Board {
    let mut cols = Vec::with_capacity(get!(NUM_COLS) as usize);
    for _ in 0..*NUM_COLS.read().unwrap() as usize {
        let mut col = Vec::with_capacity(get!(NUM_ROWS) as usize);
        for _ in 0..*NUM_ROWS.read().unwrap() as usize{
            col.push(0);
        }
        cols.push(col)
    }
    cols
}


/// Returns a Board with 3 touching cells
pub fn blinker_board() -> Board {
    let mut cols = Vec::with_capacity(get!(NUM_COLS) as usize);
    for _ in 0..*NUM_COLS.read().unwrap() as usize {
        let mut col = Vec::with_capacity(get!(NUM_ROWS) as usize);
        for _ in 0..*NUM_ROWS.read().unwrap() as usize{
            col.push(0);
        }
        cols.push(col)
    }
    cols[10][10] = 1;
    cols[10][11] = 1;
    cols[10][9] = 1;
    cols
}


pub fn load_board_from_gamefile(file: GameFile, dst_board: &mut Board){
    let file_content = get_game_file_content(file);
    let mut lines = file_content.lines();
    let mut x_offset = 0;
    let mut y_offset = 0;

    // Clear the previous board
    for col in dst_board.iter_mut() {
        for cell in col.iter_mut() {
            *cell = 0;
        }
    }

    if let Some(header) = lines.next() {
        if header.starts_with("#P") {
            let parts: Vec<&str> = header.split_whitespace().collect();
            if parts.len() == 3 {
                x_offset = parts[1].parse().unwrap_or(0);
                y_offset = parts[2].parse().unwrap_or(0);
                println!("x_offset: {}, y_offset: {}", x_offset, y_offset); // Example usage
            }
        }
    }

    // Read the file line by line and updates the board
    for (y, line) in lines.enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '*' {
                dst_board[x + 1 + x_offset][y + 1 + y_offset] = 1;
            }
        }
    }
}

pub fn load_board_from_path(path: &str, dst_board: &mut Board){
    let file = File::open(path).unwrap_or_else(|e| {
        panic!("Failed to open file {}: {}", path, e);
    });
    let mut reader = BufReader::new(file);
    let mut x_offset = 0;
    let mut y_offset = 0;

    // Clear the previous board
    for col in dst_board.iter_mut() {
        for cell in col.iter_mut() {
            *cell = 0;
        }
    }

    let mut lines = reader.lines();
    if let Some(Ok(header)) = lines.next() {
        if header.starts_with("#P") {
            let parts: Vec<&str> = header.split_whitespace().collect();
            if parts.len() == 3 {
                x_offset = parts[1].parse().unwrap_or(0);
                y_offset = parts[2].parse().unwrap_or(0);
                println!("x_offset: {}, y_offset: {}", x_offset, y_offset); // Example usage
            }
        }
    }

    // Read the file line by line and updates the board
    for (y, line) in lines.enumerate() {
        let line = line.expect("Failed to read line");
        for (x, cell) in line.split_whitespace().enumerate() {
            if cell == "*" {
                dst_board[x + 1 + x_offset][y + 1 + y_offset] = 1;
            }
        }
    }
}

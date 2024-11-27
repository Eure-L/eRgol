use crate::game_files::{get_content_from_seed, GameSeed};
use crate::globals::{NUM_COLS, NUM_ROWS};
use crate::{get, GameParams};
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Board = Vec<Vec<u8>>;


pub fn init_game(game_params: &mut GameParams, curr_board: &mut Board, next_board: &mut Board) {
    load_board_from_seed(game_params.seed.clone(), curr_board);
    load_board_from_seed(game_params.seed.clone(), next_board);
    game_params.iteration = 0;
    game_params.speed = 1;
    game_params.paused = true;
}


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


pub fn load_board_from_seed(seed: GameSeed, dst_board: &mut Board){
    let file_content = get_content_from_seed(seed);
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

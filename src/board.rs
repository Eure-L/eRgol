use crate::game_files::{get_content_from_seed, GameSeed};
use crate::globals::{NUM_COLS, NUM_ROWS};
use crate::{get, set, GameParams};
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

/// Returns an randomly composed board
pub fn new_board(x_size: u16, y_size: u16) -> Board {
    let mut rng = rand::thread_rng();
    let mut cols = Vec::with_capacity(x_size as usize);
    for _ in 0..x_size  {
        let mut col = Vec::with_capacity(y_size  as usize);
        for _ in 0..y_size {
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

fn load_board_from_lines(lines: Vec<String>,  dst_board: &mut Board) {
    let mut max_width = 0;
    let mut max_height = 0;

    // 1st: Dimensions Calculation
    let mut current_x_offset = 0;
    let mut current_y_offset = 0;

    let mut iy = 1;
    for line in &lines {
        if line.starts_with("#P") {
            // Parse offsets
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 3 {
                current_x_offset = parts[1].parse().unwrap_or(0);
                current_y_offset = parts[2].parse().unwrap_or(0);
                iy = 1;
            }
        } else if !line.is_empty() {
            // Check width and height based on the offset and content
            let width = line.len();
            max_width = max_width.max(current_x_offset + width);
            max_height = max_height.max(current_y_offset + iy);
            iy += 1;
        }
    }

    let mut new_board = new_board((max_width + 2) as u16, max_height + 2);

    // 3rd: Populate the board
    current_x_offset = 0;
    current_y_offset = 0;
    for line in lines {
        if line.starts_with("#P") {
            // Parse offsets
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 3 {
                current_x_offset = parts[1].parse().unwrap_or(0);
                current_y_offset = parts[2].parse().unwrap_or(0);
            }
        } else if !line.is_empty() {
            for (x, cell) in line.chars().enumerate() {
                if cell == '*' {
                    let x_pos = current_x_offset + x;
                    let y_pos = current_y_offset;

                    if y_pos > new_board[x].len() as u16{
                        continue;
                    }

                    new_board[1 + x_pos as usize][ 1 + y_pos as usize] = 1;
                }
            }
            current_y_offset += 1; // Move down after each line
        }
    }
    set!(NUM_COLS, max_width as u32);
    set!(NUM_ROWS, max_height as u32);

    dst_board.clone_from(&new_board);
}


pub fn load_board_from_seed(seed: GameSeed, dst_board: &mut Board)  {
    let file_content = get_content_from_seed(seed);
    let lines: Vec<String> = file_content.lines().map(|x| {x.to_string()}).collect();
    load_board_from_lines(lines, dst_board)
}

pub fn load_board_from_path(path: &str, dst_board: &mut Board){
    let file = File::open(path).unwrap_or_else(|e| {
        panic!("Failed to open file {}: {}", path, e);
    });

    let mut reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    load_board_from_lines(lines, dst_board)
}

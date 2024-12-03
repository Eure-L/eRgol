use crate::game_files::read_file_lines;
use crate::GameParams;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};
use std::path::PathBuf;

pub struct Board  {
    next_data: Vec<u8>,
    prev_data: Vec<u8>,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl Board {
    pub fn new(cols: usize, rows: usize) -> Self {
        Board {
            next_data: vec![0u8; rows * cols],
            prev_data: vec![0u8; rows * cols],
            rows,
            cols,
        }
    }
    pub fn swap_data(&mut self) {
        std::mem::swap(&mut self.prev_data, &mut self.next_data);
    }
}

impl Clone for Board{
    fn clone(&self) -> Self {
        let mut new_board = Board::new(self.rows, self.cols);
         new_board
    }
}

impl Index<(usize, usize)> for Board {
    type Output = u8;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        let index = row * self.cols + col;
        &self.prev_data[index]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        let index = row * self.cols + col;
        &mut self.next_data[index]
    }
}


pub fn init_game(game_params: &mut GameParams) -> Board{
    // Update game params to playing state
    game_params.iteration = 0;
    game_params.speed = 1;
    game_params.paused = true;

    let new_board: Board = load_board_from_seed(game_params.seed.to_str().unwrap().to_string());
    new_board
}



fn load_board_from_lines(lines: Vec<String>) -> Board {
    let b = vec![0; 55];

    let border_padding_x = 8;
    let border_padding_y = 8;

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
            max_width = max_width.max(current_x_offset + width + border_padding_x *2);
            max_height = max_height.max(current_y_offset + iy + border_padding_y *2);
            iy += 1;
        }
    }

    let mut new_board: Board= Board::new(max_width, max_height);

    // 3rd: Populate the board
    current_x_offset = 0;
    current_y_offset = 0;
    for line in lines {
        if line.starts_with("#P") {
            // Parse offsets
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 3 {
                current_x_offset = parts[1].parse().unwrap_or(0) + border_padding_x;
                current_y_offset = parts[2].parse().unwrap_or(0) + border_padding_y;
            }
        } else if !line.is_empty() {
            for (x, cell) in line.chars().enumerate() {
                if cell == '*' {
                    let x_pos = current_x_offset + x;
                    let y_pos = current_y_offset;

                    // Out of board bounds condition
                    if y_pos > new_board.rows {
                        continue;
                    }
                    new_board[(1 + x_pos as usize, 1 + y_pos as usize)] = 1;
                }
            }
            current_y_offset += 1; // Move down after each line
        }
    }
    new_board
}


pub fn load_board_from_seed(seed: String) -> Board{
    let lines: Vec<String> = read_file_lines(&*PathBuf::from(seed)).unwrap();
    load_board_from_lines(lines)
}

pub fn load_board_from_path(path: &str) -> Board{
    let file = File::open(path).unwrap_or_else(|e| {
        panic!("Failed to open file {}: {}", path, e);
    });

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    load_board_from_lines(lines)
}

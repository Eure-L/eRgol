use rand::Rng;
use eRgol::{NUM_COLS, NUM_ROWS};

pub type Board = Vec<Vec<u8>>;

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


pub fn test_board() -> Board {
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


pub fn update_board(prev_board: &mut Board, next_board: &mut Board){
    // Inner Board first as it is faster to compute
    for ix in 1..NUM_COLS -1 {
        for iy in 1..NUM_ROWS -1{
            let total_neighbors =
                prev_board[ix-1][iy-1] +
                prev_board[ix][iy-1] +
                prev_board[ix+1][iy-1] +
                prev_board[ix-1][iy] +
                prev_board[ix][iy] +
                prev_board[ix+1][iy] +
                prev_board[ix-1][iy+1] +
                prev_board[ix][iy+1] +
                prev_board[ix+1][iy+1];
            next_board[ix][iy] = match total_neighbors {
                3 => {1}
                4 => {1}
                _ => {0}
            }
        }
    }
    //Todo Edges and corners
}


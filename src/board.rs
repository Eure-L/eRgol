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

/// Retruns a board with cells at the edge
pub fn edge_board() -> Board {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS{
            col.push(0);
        }
        cols.push(col)
    }
    cols[0][1] = 1;
    cols[0][2] = 1;
    cols[0][3] = 1;
    cols[0][4] = 1;
    cols[0][5] = 1;
    cols[0][6] = 1;
    cols[0][7] = 1;
    cols[0][8] = 1;
    cols[0][9] = 1;
    cols[0][10] = 1;
    cols[0][0] = 1;
    cols[2][0] = 1;
    cols[3][0] = 1;
    cols[4][0] = 1;
    cols[5][0] = 1;
    cols[6][0] = 1;
    cols[7][0] = 1;
    cols[8][0] = 1;
    cols[9][0] = 1;
    cols[10][0] = 1;
    cols[1][0] = 1;
    cols
}


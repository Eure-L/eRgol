use crate::board::Board;
use crate::globals::{NUM_COLS, NUM_ROWS};

/// Updates a given board prev_board to in the next_board


pub enum Kernels {
    CpuSequential,
    CpuSequentialTiled,
    CpuMultiThreads,
    GPU,
}

impl Kernels {
    pub(crate) fn clone(&self) -> Kernels {
        match self {
            Kernels::CpuSequential => Kernels::CpuSequential,
            Kernels::CpuSequentialTiled => Kernels::CpuSequentialTiled,
            Kernels::CpuMultiThreads => Kernels::CpuMultiThreads,
            Kernels::GPU => Kernels::GPU,
        }
    }
}

pub fn get_kernel_func(kernel: Kernels) -> fn(&mut Board, &mut Board) {
    match kernel {
        Kernels::CpuSequential => update_board,
        Kernels::CpuSequentialTiled => panic!("Not implemented"),
        Kernels::CpuMultiThreads => panic!("Not implemented"),
        Kernels::GPU => panic!("Not implemented"),
        _ => update_board
    }
}

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

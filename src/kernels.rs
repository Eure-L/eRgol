use crate::board::Board;

/// Updates a given board previous_board to in the next_board

#[derive(Debug)]
pub enum Kernels {
    CpuSequential
}

impl Kernels {
    pub(crate) fn clone(&self) -> Kernels {
        match self {
            Kernels::CpuSequential => Kernels::CpuSequential
        }
    }
}

pub fn get_kernel_func(kernel: Kernels) -> fn(&mut Board) {
    match kernel {
        Kernels::CpuSequential => update_board
    }
}

pub fn update_board(board: &mut Board) {
    // Inner board computation as the edges are off
    for ix in 1..board.cols - 1 {
        for iy in 1..board.rows - 1 {
            let mut total_neighbors = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx != 0 || dy != 0 {
                        let x = (ix as i32 + dx);
                        let y = (iy as i32 + dy);
                        total_neighbors += board[(x as usize, y as usize)];
                    }
                }
            }
            board[(ix, iy)] = match (board[(ix, iy)], total_neighbors) {
                (1, 2 | 3) => 1,
                (_, 3) => 1,
                _ => 0,
            };
        }
    }
}

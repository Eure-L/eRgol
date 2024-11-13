use crate::board::Board;
use crate::globals::{BRAILLE_ALPHABET_START, BRAILLE_SIZE_X, BRAILLE_SIZE_Y, NUM_BRAILLE_BLOCS_X, NUM_BRAILLE_BLOCS_Y};
use crate::kernels::Kernels::CpuSequential;
use crate::{board, get, Game, GameModes, GameParams};
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::{Stdout, Write};
use std::sync::mpsc::Receiver;
use crate::game_files::GameSeed;

/// Renders the grid by computing corresponding braille character for each patch of alive/dead cells
/// Uses Char arithmetic to render the correct Braille Unicode character, making it unsafe.
pub(crate) unsafe fn render_braille(stdout: &mut Stdout, prev_board: &Board, game_params: & GameParams, forced: bool) {
    if forced
    {
        stdout.queue(SetForegroundColor(Color::Yellow)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Reset)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    const BASE: u32 = 2;
    // Braill blocks iteration
    for bloc_x in 0..get!(NUM_BRAILLE_BLOCS_X) - 1 {
        for bloc_y in 0..get!(NUM_BRAILLE_BLOCS_Y) - 1 {
            let mut code = BRAILLE_ALPHABET_START; // Code of corresponding unicode char for this brail code
            for ix in 0..BRAILLE_SIZE_X {
                let x = bloc_x * BRAILLE_SIZE_X + ix;

                // First 6 Braille cells pattern computing
                for iy in 0..BRAILLE_SIZE_Y - 1 {
                    let y = bloc_y * BRAILLE_SIZE_Y + iy;
                    let weight = match prev_board[x as usize][y as usize] {
                        1 => { BASE.pow((iy + ix * (BRAILLE_SIZE_Y - 1)) as u32) }
                        _ => { 0 }
                    };
                    code = code + weight;
                }

                // Two last Braille cells
                let weight = match prev_board[x as usize][(bloc_y * (BRAILLE_SIZE_Y) + BRAILLE_SIZE_Y - 1) as usize] {
                    1 => { (ix as u32 + 1) * 0x40 }
                    _ => { 0 }
                };
                code = code + weight;
            }
            stdout.queue(MoveTo(bloc_x as u16, bloc_y as u16)).unwrap();
            print!("{}", char::from_u32_unchecked(code));
        }
    }
    stdout.queue(SetBackgroundColor(Color::Reset)).unwrap();
    stdout.queue(MoveTo(0, get!(NUM_BRAILLE_BLOCS_Y) as u16)).unwrap();
    print!("Iteration:{}        ", game_params.iteration);
    stdout.queue(MoveTo(0, get!(NUM_BRAILLE_BLOCS_Y) as u16 + 1)).unwrap();
    print!("Speed:{}            ", game_params.speed);
    stdout.queue(MoveTo(0, get!(NUM_BRAILLE_BLOCS_Y) as u16 + 2)).unwrap();
    print!("Pause:{}            ", game_params.paused);
    stdout.queue(MoveTo(0, get!(NUM_BRAILLE_BLOCS_Y) as u16 + 3)).unwrap();
    print!("m: menu | p: pause | r: reset | s: step | +: speed up | -: slow down");
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    stdout.flush().unwrap()
}

pub(crate) unsafe fn render_menu(stdout: &mut Stdout, prev_board: &Board, game_params: & GameParams) {

    stdout.queue(SetForegroundColor(Color::DarkYellow)).unwrap();
    stdout.queue(SetBackgroundColor(Color::DarkGrey)).unwrap();

    let X_CORNER : u16 = (4 * get!(NUM_BRAILLE_BLOCS_X) / 9 - 1) as u16;
    let Y_CORNER : u16 = (4 * get!(NUM_BRAILLE_BLOCS_Y) / 9 - 1) as u16;

    stdout.queue(MoveTo(X_CORNER, Y_CORNER - 1));
    print!("===         Seeds                   ===");
    stdout.queue(MoveTo(X_CORNER, Y_CORNER));
    print!("===         1 - Pulsar              ===");
    stdout.queue(MoveTo(X_CORNER, Y_CORNER + 1));
    print!("===         2 - Braille             ===");
    stdout.queue(MoveTo(X_CORNER, Y_CORNER + 2));
    print!("===         Controlls               ===");
    stdout.queue(MoveTo(X_CORNER, Y_CORNER + 3));
    print!("===         ' ': pause game         ===");
    stdout.queue(MoveTo(X_CORNER, Y_CORNER + 4));
    print!("===         's': single step        ===");
    stdout.queue(MoveTo(X_CORNER, Y_CORNER + 5));
    print!("===         r: reset                ===");
    stdout.queue(MoveTo(X_CORNER, Y_CORNER + 6));
    print!("===         m: change menu          ===");


    stdout.queue(SetForegroundColor(Color::Yellow)).unwrap();
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();

    // stdout.flush().unwrap()
}


pub(crate) unsafe fn rendering_tread(render_rx: &Receiver<Game>) {
    let mut stdout = std::io::stdout();
    let mut prev_board = board::empty_board();
    render_braille(&mut stdout,
                   &prev_board,
                   &mut GameParams {
                       iteration: 0,
                       speed: 1,
                       paused: true,                // "
                       mode: GameModes::Playing,    // Doesnt matter to the rendering thread
                       seed: GameSeed::Braille,     // "
                       kernel: CpuSequential,       // "
                   },
                   true);
    loop {
        let curr_game: Game = match render_rx.recv() {
            Ok(rcv_game) => { rcv_game }
            Err(_) => break,
        };
        let curr_board = curr_game.board;
        let game_params = curr_game.game_params;
        match game_params.mode {
            GameModes::Playing => { render_braille(&mut stdout, &prev_board, & game_params, false); }

            GameModes::MainMenu => {
                render_menu(&mut stdout, &prev_board, & game_params);
            }
        }
        prev_board = curr_board;
    }
}
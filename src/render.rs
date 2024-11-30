use crate::board::Board;
use crate::game_files::GameSeed;
use crate::game_structs::{Game, Rendering, DEFAULT_GAME_PARAMS};
use crate::globals::{get_rendering_xsize, get_rendering_ysize, BRAILLE_ALPHABET_START, BRAILLE_SIZE_X, BRAILLE_SIZE_Y, COLOR_FONT, NUM_BRAILLE_BLOCS_X, NUM_BRAILLE_BLOCS_Y, NUM_COLS, NUM_ROWS};
use crate::ui::{Drawable, InterractiveDrawable, InterractiveTextBox, TextBox};
use crate::{get, GameModes, GameParams};
use crossterm::cursor::MoveTo;
use crossterm::style::{Attribute, Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::Stdout;
use std::sync::mpsc::Receiver;
use strum::IntoEnumIterator;


unsafe fn braille_rendering(stdout: &mut Stdout, board: &Board){
    const BASE: u32 = 2;
    // Braill blocks iteration
    for bloc_x in 0..get_rendering_xsize() {
        for bloc_y in 0..get_rendering_ysize(){

            let mut code = BRAILLE_ALPHABET_START; // Code of corresponding unicode char for this brail code
            for ix in 0..BRAILLE_SIZE_X {
                let x = bloc_x * BRAILLE_SIZE_X + ix;
                if x >= get!(NUM_COLS){
                    break;
                }

                // First 6 Braille cells pattern computing
                for iy in 0..BRAILLE_SIZE_Y - 1 {
                    let y = bloc_y * BRAILLE_SIZE_Y + iy;
                    if y >= get!(NUM_ROWS){
                        break;
                    }

                    let weight = match board[x as usize][y as usize] {
                        1 => { BASE.pow(iy + ix * (BRAILLE_SIZE_Y - 1)) }
                        _ => { 0 }
                    };
                    code = code + weight;
                }

                // Two last Braille cells
                let y = bloc_y * (BRAILLE_SIZE_Y) + BRAILLE_SIZE_Y - 1;
                if y >= get!(NUM_ROWS){
                    break;
                }
                let weight = match board[x as usize][y as usize] {
                    1 => { (ix + 1) * 0x40 }
                    _ => { 0 }
                };

                code = code + weight;
            }
            stdout.queue(MoveTo(bloc_x as u16, bloc_y as u16)).unwrap();
            stdout.queue(Print(char::from_u32_unchecked(code))).unwrap();
        }
    }
}

unsafe fn render_game_board(stdout: &mut Stdout, board: &Board, game_params: &GameParams){
    match game_params.rendering {
        Rendering::Braille => {
            braille_rendering(stdout, board);
        }
    }
}

fn render_game_ui(stdout: &mut Stdout, game_params: &GameParams, forced :bool){


    let pause_str = if game_params.paused {
        "Paused_"
    } else {
        "Playing"
    }.to_string();

    let text = vec![
        format!("{}",pause_str),
        format!("Step n°: {}",game_params.iteration),
        format!("Speed:   {}",game_params.speed),
        format!("Game seed:   {}",game_params.seed),
        format!("id:   {}",game_params.menu_scroll),
        format!("Rendering Mode: {}",game_params.rendering),
        format!("Computing Kernel: {:?}",game_params.kernel),
        "m: menu | p: pause | r: reset | s: step | +: speed up | -: slow down".to_string()
    ];

    let controlls_box = TextBox{
        header: format!("Rendering: {} | Computing: {:?}", game_params.rendering, game_params.kernel),
        header_color: Color::White,
        header_attribute: Attribute::Bold,
        text : text,
        text_color: Color::White,
        text_attribute: Attribute::Reset,
        background_color: Color::Reset,
    };

    controlls_box.draw_at(stdout, 0, get_rendering_ysize() as u16, forced);
}


pub(crate) unsafe fn render_board(stdout: &mut Stdout, board: &Board, game_params: &GameParams, forced: bool) {
    if forced
    {
        stdout.queue(SetForegroundColor(Color::Yellow)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Reset)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    stdout.queue(SetForegroundColor(Color::Yellow)).unwrap();
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    render_game_board(stdout, board, game_params);
    stdout.queue(SetForegroundColor(Color::Reset)).unwrap();
    render_game_ui(stdout, game_params, forced);
}

pub(crate) fn render_menu(stdout: &mut Stdout, game_params: &GameParams, forced: bool) {

    let x_corner: u16 = (2 * get!(NUM_BRAILLE_BLOCS_X) / 9 - 1) as u16;
    let y_corner: u16 = (2 * get!(NUM_BRAILLE_BLOCS_Y) / 9 - 1) as u16;

    let seed_box = InterractiveTextBox{
        header: "Available SEEDS".parse().unwrap(),
        header_color: COLOR_FONT,
        header_attribute: Attribute::Bold,
        text : GameSeed::iter().map(|x| {x.to_string()}).collect(),
        text_color: COLOR_FONT,
        text_attribute: Attribute::NoBold,
        background_color: Color::DarkYellow,
    };
    
    let controlls_box = TextBox{
        header: "Press 'i' to type seed id or .life file path:".to_string(),
        header_color: COLOR_FONT,
        header_attribute: Attribute::Bold,
        text : vec![ "Input here".into()],
        text_color: Color::Grey,
        text_attribute: Attribute::NoBold,
        background_color: Color::Yellow,
    };

    controlls_box.draw_at(stdout, x_corner, y_corner, forced);
    seed_box.draw_with_input(stdout, x_corner, y_corner + 8, game_params, forced);

}

pub(crate) unsafe fn rendering_tread(render_rx: &Receiver<Game>) {
    let mut stdout = std::io::stdout();
    let mut prev_game_params: GameParams = DEFAULT_GAME_PARAMS.clone();

    let current_game: Game = match render_rx.recv() {
        Ok(rcv_game) => { rcv_game }
        Err(_) => return,
    };

    let previous_board = current_game.board;
    render_board(&mut stdout, &previous_board, &prev_game_params, true);

    loop {
        let current_game: Game = match render_rx.recv() {
            Ok(rcv_game) => { rcv_game }
            Err(_) => break,
        };

        let game_params = current_game.game_params.clone();
        let mut forced = false;

        if game_params.mode != prev_game_params.mode {
            forced = true;
        }

        match game_params.mode {
            GameModes::Playing => {
                render_board(&mut stdout, &current_game.board, &game_params, forced);
            }
            GameModes::MainMenu => {
                render_menu(&mut stdout, &game_params, forced);
            }
        }
        prev_game_params = game_params.clone();
    }
}
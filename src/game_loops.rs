use crate::board::{init_game, Board};
use crate::kernels::get_kernel_func;
use crate::{GameModes, GameParams};
use crossterm::event::{Event, KeyCode};
use std::sync::mpsc::Sender;
use std::time::Duration;
use crate::game_files::GameSeed;
use crate::game_structs::Game;

fn step(update_fun: fn(&mut Board, &mut Board), curr_board: &mut Board, next_board: &mut Board) {
    update_fun(curr_board, next_board);
    std::mem::swap(curr_board, next_board);
}

pub fn game_menu(game_params: &mut GameParams,
             curr_board: &mut Board,
             next_board: &mut Board,
             render_tx: Sender<Game>) -> Result<(), String>
{
    let update_fun = get_kernel_func(game_params.clone().kernel);

    while crossterm::event::poll(Duration::from_millis(7)).unwrap_or(false) {
        if let Ok(Event::Key(key_event)) = crossterm::event::read() {
            match key_event.code {
                KeyCode::Backspace => { game_params.paused = !game_params.paused }
                KeyCode::Esc | KeyCode::Char('q') => {
                    return Err("user quit".parse().unwrap());
                }
                // Pause/unpause
                KeyCode::Char('m') => {
                    game_params.mode = GameModes::Playing;
                }                    // Pause/unpause
                KeyCode::Char('s') => {
                    step(update_fun, curr_board, next_board)
                }                // Pause/unpause
                KeyCode::Char('r') => {
                    game_params.mode = GameModes::Playing;
                    init_game(game_params, curr_board, next_board);
                }
                KeyCode::Char('1') => {
                    game_params.seed = GameSeed::Pulsar;
                    init_game(game_params, curr_board , next_board);
                    game_params.mode = GameModes::Playing;
                }
                KeyCode::Char('2') => {
                    game_params.seed = GameSeed::Braille;
                    init_game(game_params, curr_board, next_board);
                    game_params.mode = GameModes::Playing;
                }
                KeyCode::Char('3') => {
                    game_params.seed = GameSeed::GliderGun;
                    init_game(game_params, curr_board, next_board);
                    game_params.mode = GameModes::Playing;
                }
                KeyCode::Char('4') => {
                    game_params.seed = GameSeed::Spaceship;
                    init_game(game_params, curr_board, next_board);
                    game_params.mode = GameModes::Playing;
                }
                KeyCode::Char('5') => {
                    game_params.seed = GameSeed::Oscillator;
                    init_game(game_params, curr_board, next_board);
                    game_params.mode = GameModes::Playing;
                }
                KeyCode::Char('6') => {
                    game_params.seed = GameSeed::SpaceshipFactory;
                    init_game(game_params, curr_board, next_board);
                    game_params.mode = GameModes::Playing;
                }
                KeyCode::Char('7') => {
                    game_params.seed = GameSeed::UnitCell;
                    init_game(game_params, curr_board, next_board);
                    game_params.mode = GameModes::Playing;
                }
                KeyCode::Char('8') => {
                    game_params.seed = GameSeed::HERSHEL;
                    init_game(game_params, curr_board, next_board);
                    game_params.mode = GameModes::Playing;
                }
                KeyCode::Char('9') => {
                    game_params.seed = GameSeed::RLE28;
                    init_game(game_params, curr_board, next_board);
                    game_params.mode = GameModes::Playing;
                }
                _ => {}
            }
        }
    }

    if !game_params.paused {
        for step_id in 0..game_params.speed {
            step(update_fun, curr_board, next_board);
        }
    }

    // Asks to the rendering thread to draw the frame :)
    let _ = render_tx.send(Game { game_params: game_params.clone(), board: curr_board.clone() });
    std::thread::sleep(Duration::from_millis(5));

    Ok(())
}

pub fn play(game_params: &mut GameParams,
            curr_board: &mut Board,
            next_board: &mut Board,
            render_tx: Sender<Game>) -> Result<(), String>
{
    let update_fun = get_kernel_func(game_params.clone().kernel);

    while crossterm::event::poll(Duration::from_millis(7)).unwrap_or(false) {
        if let Ok(Event::Key(key_event)) = crossterm::event::read() {
            match key_event.code {
                KeyCode::Backspace => { game_params.paused = !game_params.paused }
                KeyCode::Esc | KeyCode::Char('q') => {
                    return Err("user quit".parse().unwrap());
                }
                // unit step
                KeyCode::Char('s') => {
                    step(update_fun, curr_board, next_board)
                }
                // Speed up
                KeyCode::Char('+') => {
                    if game_params.speed < 16 {
                        game_params.speed = game_params.speed * 2
                    }
                }
                // Slow down
                KeyCode::Char('-') => {
                    if game_params.speed > 1 {
                        game_params.speed = game_params.speed / 2
                    }
                }
                // Pause/unpause
                KeyCode::Char('p') => {
                    game_params.paused = !game_params.paused;
                }
                // Pause/unpause
                KeyCode::Char('r') => {
                    init_game(game_params, curr_board, next_board);
                }
                // Pause/unpause
                KeyCode::Char('m') => {
                    game_params.mode = GameModes::MainMenu;
                }
                KeyCode::Char(' ') => {
                    game_params.paused = !game_params.paused;
                    break;
                }
                _ => {}
            }
        }
    }

    if !game_params.paused {
        for step_id in 0..game_params.speed {
            step(update_fun, curr_board, next_board);
        }
    }
    // Asks to the rendering thread to draw the frame :)
    let _ = render_tx.send(Game { game_params: game_params.clone(), board: curr_board.clone() });
    std::thread::sleep(Duration::from_millis(2));

    if !game_params.paused {
        game_params.iteration += game_params.speed;
    }
    Ok(())
}
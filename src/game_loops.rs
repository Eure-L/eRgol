use crate::board::{init_game, Board};
use crate::game_files::get_seed_file_from_index;
use crate::game_structs::Game;
use crate::kernels::get_kernel_func;
use crate::{GameModes, GameParams};
use crossterm::event::{Event, KeyCode, MouseEventKind};
use std::sync::mpsc::Sender;
use std::time::Duration;

fn step(update_fun: fn(&mut Board), board: &mut Board) {
    update_fun(board);
    board.swap_data()
}

pub fn game_menu(game_params: &mut GameParams,
                 board: &mut Board,
                 render_tx: Sender<Game>) -> Result<(), String>
{
    let update_fun = get_kernel_func(game_params.clone().kernel);

    while crossterm::event::poll(Duration::from_millis(7)).unwrap_or(false) {
        if let Ok(event) = crossterm::event::read() {
            match event {
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::ScrollUp => {
                            game_params.menu_scroll -= 1;
                        }
                        MouseEventKind::ScrollDown => {
                            game_params.menu_scroll += 1;
                        }
                        _ => {}
                    }
                }
                Event::Key(key_event) => {
                    match key_event.code {
                        KeyCode::Backspace => {
                            game_params.paused = !game_params.paused;
                        }
                        KeyCode::Esc | KeyCode::Char('q') => {
                            return Err("user quit".parse().unwrap());
                        }
                        KeyCode::Char('m') => {
                            game_params.mode = GameModes::Playing;
                        }
                        KeyCode::Char('s') => {
                            step(update_fun, board);
                        }
                        KeyCode::Char('r') => {
                            game_params.mode = GameModes::Playing;
                            *board = init_game(game_params);
                        }
                        KeyCode::Down => {
                            game_params.menu_scroll += 1;
                        }
                        KeyCode::Up => {
                            if game_params.menu_scroll > 0 {
                                game_params.menu_scroll -= 1;
                            }
                        }
                        KeyCode::Enter => {
                            game_params.seed = get_seed_file_from_index(game_params.menu_scroll).parse().unwrap();
                            game_params.mode = GameModes::Playing;
                            *board = init_game(game_params);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    if !game_params.paused {
        for _step_id in 0..game_params.speed {
            step(update_fun, board);
        }
    }

    // Asks to the rendering thread to draw the frame :)
    let _ = render_tx.send(Game { game_params: game_params.clone(), board: board.clone() });
    std::thread::sleep(Duration::from_millis(5));

    Ok(())
}

pub fn play(game_params: &mut GameParams,
            board: &mut Board,
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
                    step(update_fun, board)
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
                    *board = init_game(game_params);
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
        for _step_id in 0..game_params.speed {
            step(update_fun, board);
        }
    }
    // Asks to the rendering thread to draw the frame :)
    let _ = render_tx.send(Game { game_params: game_params.clone(), board: board.clone() });
    std::thread::sleep(Duration::from_millis(2));

    if !game_params.paused {
        game_params.iteration += game_params.speed;
    }
    Ok(())
}
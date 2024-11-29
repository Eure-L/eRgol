use std::cmp;
use std::cmp::min;
use lazy_static::lazy_static;
use terminal_size::{Width, Height, terminal_size};
use std::sync::RwLock;
use crossterm::style::Color;
use crate::get;

pub const BRAILLE_SIZE_X: u32 = 2;
pub const BRAILLE_SIZE_Y: u32 = 4;

pub const MAX_WIDTH: u32 = 1000;  // Set your desired max width
pub const MAX_HEIGHT: u32 = 1000; // Set your desired max height

pub fn get_terminal_ysize() -> u32 {
    terminal_size().map(|(_, Height(h))| h).unwrap_or(42) as u32 - 5
}

pub fn get_terminal_xsize() -> u32{
        terminal_size().map(|(Width(w), _)| w).unwrap_or(42) as u32 - 5
}
pub fn get_rendering_ysize() -> u32 {
    let max_brail_height = 1 + get!(NUM_ROWS) / BRAILLE_SIZE_Y;
    // min(get_terminal_ysize() , 10)
    max_brail_height
}

pub fn get_rendering_xsize() -> u32{
    let max_brail_width = 1 + get!(NUM_COLS) / BRAILLE_SIZE_X;
    // min(get_terminal_xsize() , 20)
    max_brail_width
}

lazy_static! {
    pub static ref NUM_BRAILLE_BLOCS_Y: RwLock<u32> = RwLock::new(
        cmp::min(terminal_size().map(|(_, Height(h))| h).unwrap_or(42) as u32 - 5, MAX_HEIGHT)
    );
    pub static ref NUM_BRAILLE_BLOCS_X: RwLock<u32> = RwLock::new(
        cmp::min(terminal_size().map(|(Width(w), _)| w).unwrap_or(42) as u32, MAX_WIDTH)
    );

    //
    pub static ref INIT_BOARD_SIZE_Y: RwLock<u32> = RwLock::new(
        terminal_size().map(|(_, Height(h))| h).unwrap_or(42) as u32 - 5 * BRAILLE_SIZE_Y
    );
    pub static ref INIT_BOARD_SIZE_X: RwLock<u32> = RwLock::new(
        terminal_size().map(|(Width(w), _)| w).unwrap_or(42) as u32 * BRAILLE_SIZE_X
    );

    //
    pub static ref NUM_COLS: RwLock<u32> = RwLock::new(
        get!(INIT_BOARD_SIZE_X)
    );
    pub static ref NUM_ROWS: RwLock<u32> = RwLock::new(
        get!(INIT_BOARD_SIZE_Y)
    );
}

pub const BRAILLE_ALPHABET_START: u32 = 0x2800;
pub const BG_GAME: Color = Color::Reset;
pub const BG_MENU: Color = Color::DarkYellow;
pub const COLOR_FONT: Color = Color::Black;



// 280x 	⠀ 	⠁ 	⠂ 	⠃ 	⠄ 	⠅ 	⠆ 	⠇ 	⠈ 	⠉ 	⠊ 	⠋ 	⠌ 	⠍ 	⠎ 	⠏
// U+281x 	⠐ 	⠑ 	⠒ 	⠓ 	⠔ 	⠕ 	⠖ 	⠗ 	⠘ 	⠙ 	⠚ 	⠛ 	⠜ 	⠝ 	⠞ 	⠟
// U+282x 	⠠ 	⠡ 	⠢ 	⠣ 	⠤ 	⠥ 	⠦ 	⠧ 	⠨ 	⠩ 	⠪ 	⠫ 	⠬ 	⠭ 	⠮ 	⠯
// U+283x 	⠰ 	⠱ 	⠲ 	⠳ 	⠴ 	⠵ 	⠶ 	⠷ 	⠸ 	⠹ 	⠺ 	⠻ 	⠼ 	⠽ 	⠾ 	⠿
// U+284x 	⡀ 	⡁ 	⡂ 	⡃ 	⡄ 	⡅ 	⡆ 	⡇ 	⡈ 	⡉ 	⡊ 	⡋ 	⡌ 	⡍ 	⡎ 	⡏
// U+285x 	⡐ 	⡑ 	⡒ 	⡓ 	⡔ 	⡕ 	⡖ 	⡗ 	⡘ 	⡙ 	⡚ 	⡛ 	⡜ 	⡝ 	⡞ 	⡟
// U+286x 	⡠ 	⡡ 	⡢ 	⡣ 	⡤ 	⡥ 	⡦ 	⡧ 	⡨ 	⡩ 	⡪ 	⡫ 	⡬ 	⡭ 	⡮ 	⡯
// U+287x 	⡰ 	⡱ 	⡲ 	⡳ 	⡴ 	⡵ 	⡶ 	⡷ 	⡸ 	⡹ 	⡺ 	⡻ 	⡼ 	⡽ 	⡾ 	⡿
// U+288x 	⢀ 	⢁ 	⢂ 	⢃ 	⢄ 	⢅ 	⢆ 	⢇ 	⢈ 	⢉ 	⢊ 	⢋ 	⢌ 	⢍ 	⢎ 	⢏
// U+289x 	⢐ 	⢑ 	⢒ 	⢓ 	⢔ 	⢕ 	⢖ 	⢗ 	⢘ 	⢙ 	⢚ 	⢛ 	⢜ 	⢝ 	⢞ 	⢟
// U+28Ax 	⢠ 	⢡ 	⢢ 	⢣ 	⢤ 	⢥ 	⢦ 	⢧ 	⢨ 	⢩ 	⢪ 	⢫ 	⢬ 	⢭ 	⢮ 	⢯
// U+28Bx 	⢰ 	⢱ 	⢲ 	⢳ 	⢴ 	⢵ 	⢶ 	⢷ 	⢸ 	⢹ 	⢺ 	⢻ 	⢼ 	⢽ 	⢾ 	⢿
// U+28Cx 	⣀ 	⣁ 	⣂ 	⣃ 	⣄ 	⣅ 	⣆ 	⣇ 	⣈ 	⣉ 	⣊ 	⣋ 	⣌ 	⣍ 	⣎ 	⣏
// U+28Dx 	⣐ 	⣑ 	⣒ 	⣓ 	⣔ 	⣕ 	⣖ 	⣗ 	⣘ 	⣙ 	⣚ 	⣛ 	⣜ 	⣝ 	⣞ 	⣟
// U+28Ex 	⣠ 	⣡ 	⣢ 	⣣ 	⣤ 	⣥ 	⣦ 	⣧ 	⣨ 	⣩ 	⣪ 	⣫ 	⣬ 	⣭ 	⣮ 	⣯
// U+28Fx 	⣰ 	⣱ 	⣲ 	⣳ 	⣴ 	⣵ 	⣶ 	⣷ 	⣸ 	⣹ 	⣺ 	⣻ 	⣼ 	⣽ 	⣾ 	⣿
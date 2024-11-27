use std::cmp;
use lazy_static::lazy_static;
use terminal_size::{Width, Height, terminal_size};
use std::sync::RwLock;
use crossterm::style::Color;
use crate::get;

pub const BRAILLE_SIZE_X: u32 = 2;
pub const BRAILLE_SIZE_Y: u32 = 4;

pub const MAX_WIDTH: u32 = 1000;  // Set your desired max width
pub const MAX_HEIGHT: u32 = 1000; // Set your desired max height


lazy_static! {
    pub static ref NUM_BRAILLE_BLOCS_Y: RwLock<u32> = RwLock::new(
        cmp::min(terminal_size().map(|(_, Height(h))| h).unwrap_or(42) as u32 - 5, MAX_HEIGHT)
    );
    pub static ref NUM_BRAILLE_BLOCS_X: RwLock<u32> = RwLock::new(
        cmp::min(terminal_size().map(|(Width(w), _)| w).unwrap_or(42) as u32, MAX_WIDTH)
    );
    pub static ref NUM_COLS: RwLock<u32> = RwLock::new(
        get!(NUM_BRAILLE_BLOCS_X) * BRAILLE_SIZE_X
    );
    pub static ref NUM_ROWS: RwLock<u32> = RwLock::new(
        get!(NUM_BRAILLE_BLOCS_Y) * BRAILLE_SIZE_Y
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
use sdl2::pixels::Color;

pub const PIXELS_PER_TILE: usize = 40;
/// <summary>
/// This number gets muliplied by PIXELS_PER_TILE to show the grid past the tiles. The higher the number the smaler the grid. Values should be between 0 and 1.
/// </summary>
pub const TILE_BOARDER_OFFSET: f32 = 0.8;

// set standard colors for items
pub const BACKGROUND_COLOR: Color = Color::YELLOW;
pub const TILE_COLOR_MINE: Color = Color::BLACK;
pub const TILE_COLOR_UNOPENED: Color = Color::GRAY;
pub const TILE_COLOR_FLAGED: Color = Color::CYAN;

pub fn tile_color_opened(number: i32) -> Color {
    match number {
        0 => BACKGROUND_COLOR,
        1 => Color::RGB(255, 165, 0),
        2 => Color::RGB(255, 83, 0),
        3 => Color::RGB(191, 64, 191),
        4 => Color::RGB(93, 63, 211),
        5 => Color::RGB(48, 25, 52),
        6 => Color::RGB(1, 50, 32),
        7 => Color::RGB(2, 113, 72),
        8 => Color::RGB(4, 175, 112),
        _ => panic!("Mine number higher than 8. number: {}", number), // should not occur
    }
}

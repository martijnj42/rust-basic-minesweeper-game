use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use super::game_struct::{Grid, Tile, TileType};
use super::visual_settings;

pub struct Renderer {
    pub canvas: WindowCanvas,
}
impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), String> {
        let x_pos = (x * visual_settings::PIXELS_PER_TILE) as i32;
        let y_pos = (y * visual_settings::PIXELS_PER_TILE) as i32;

        let tile_size =
            (visual_settings::PIXELS_PER_TILE as f32 * visual_settings::TILE_BOARDER_OFFSET) as u32;

        self.canvas.set_draw_color(color);
        self.canvas
            .fill_rect(Rect::new(x_pos, y_pos, tile_size, tile_size))?;

        Ok(())
    }

    fn tile_to_color(&self, tile: &Tile, cheat: bool) -> Color {
        if tile.flagged {
            return visual_settings::TILE_COLOR_FLAGED;
        }
        match tile.status {
            TileType::Mine if cheat => visual_settings::TILE_COLOR_MINE,
            TileType::Mine => visual_settings::TILE_COLOR_UNOPENED,
            TileType::Unopened => visual_settings::TILE_COLOR_UNOPENED,
            TileType::Opened => visual_settings::tile_color_opened(tile.number),
        }
    }

    pub fn draw_grid(&mut self, grid: &Grid, cheat: bool) -> Result<(), String> {
        // draw background
        let background_color = visual_settings::BACKGROUND_COLOR;
        self.canvas.set_draw_color(background_color);
        self.canvas.clear();

        for x in 0..grid.single_size {
            for y in 0..grid.single_size {
                let tile_color = self.tile_to_color(
                    grid.get_tile_by_location(&super::game_struct::Location { x, y }),
                    cheat,
                );

                self.draw_pixel(x, y, tile_color)?;
            }
        }

        self.canvas.present();

        Ok(())
    }
}

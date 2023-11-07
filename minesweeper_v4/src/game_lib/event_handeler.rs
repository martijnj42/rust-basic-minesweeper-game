use super::game_struct::Location;
use super::visual_settings;

pub struct EventHandeler {}
impl EventHandeler {
    pub fn calculate_tile_clicked(grid_size: usize, x: usize, y: usize) -> Result<Location, ()> {
        if (x > grid_size * visual_settings::PIXELS_PER_TILE)
            || (y > grid_size * visual_settings::PIXELS_PER_TILE)
        {
            return Err(());
        }

        Ok(Location {
            x: x / visual_settings::PIXELS_PER_TILE,
            y: y / visual_settings::PIXELS_PER_TILE,
        })
    }
}

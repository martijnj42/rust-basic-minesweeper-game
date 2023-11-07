use super::game_struct::{Grid, Location, Tile, TileType};

extern crate rand;

use rand::seq::SliceRandom;

pub struct Game {
    pub grid: Grid,
    pub amount_of_mines: usize,
}
impl Game {
    /// <summary>
    /// Initiate a new grid with mines and size gridsize.
    /// </summary>
    pub fn new_game(grid_size: usize, mines: usize) -> Game {
        fn place_random_mines(grid: &mut Grid, grid_size: usize, mine_amount: usize) {
            let mut mine_ids: Vec<usize> = (0..grid_size * grid_size).collect();
            mine_ids.shuffle(&mut rand::thread_rng());
            mine_ids.truncate(mine_amount);

            for id in mine_ids {
                grid.change_tile_status_by_id(id, TileType::Mine);

                for neighbour in grid.get_neighbours(&id) {
                    grid.change_tile_number_by_id(neighbour, grid.grid[neighbour].number + 1);
                }
            }
        }

        if grid_size > 20 {
            println!(
                "Warning grid very large grid: {} x {}",
                grid_size, grid_size
            );
        }

        if mines > grid_size * grid_size {
            panic!(
                "Too many mines for the field: {}. Field size {} x {}",
                mines, grid_size, grid_size
            );
        }

        let mut new_game = Game {
            grid: Grid::new_grid(grid_size),
            amount_of_mines: mines,
        };

        // println!("Created grid, placing mines...");
        place_random_mines(&mut new_game.grid, grid_size, mines);
        // println!("Mine field finished, let the game begin!");

        new_game
    }

    /// <summary>
    /// Restart a new random game with the same grid size and mines.
    /// </summary>
    pub fn restart_game(&mut self) {
        *self = Game::new_game(self.grid.single_size, self.amount_of_mines);
    }

    /// <summary>
    /// Sweeps the grid, first checking the tile then its adjacent tiles. Openes the ones which have a number of 0 and first adjacent with a number.
    /// </summary>
    pub fn sweep_grid(&mut self, start_tile: &Location) {
        /// <summary>
        /// Checks the tile type and number to see if the sweep should continue with that tile
        /// </summary>
        fn check_tile(tile: &Tile) -> Result<(), ()> {
            match tile.status {
                TileType::Mine | TileType::Opened => Err(()),
                TileType::Unopened if tile.number == 0 => Ok(()),
                TileType::Unopened => Err(()),
            }
        }

        // check first tile
        // get neighbours
        // while unchecked neighbours
        //      get neighbours
        //      check neighbours

        let mut tiles_to_open: Vec<usize> = vec![self.grid.location_to_id(start_tile)];
        let mut tile_check_index: usize = 0;

        while tile_check_index < tiles_to_open.len() {
            let current_tile = tiles_to_open[tile_check_index];

            let tile_check_result = check_tile(&self.grid.grid[current_tile]);
            if tile_check_result.is_ok() {
                let neighbours: Vec<usize> = self.grid.get_neighbours(&current_tile);

                for neighbour in neighbours {
                    if !tiles_to_open.contains(&neighbour) {
                        tiles_to_open.push(neighbour);
                    }
                }
            }

            tile_check_index += 1;
        }

        // open all found tiles
        for tile in tiles_to_open {
            self.grid.change_tile_status_by_id(tile, TileType::Opened);
        }
    }

    /// <summary>
    /// Sweeps the grid, first checking the tile then its adjacent tiles. Openes the ones which have a number of 0 and first adjacent with a number. Returns 1 if game still playing, 0 if won and None if dead.
    /// </summary>
    pub fn user_input(&mut self, decision: Decision) -> Option<bool> {
        if decision.flag {
            let flag = self.grid.get_tile_by_location(&decision.location).flagged;
            self.grid
                .change_tile_flag_by_location(decision.location, !flag);

            return Some(true);
        }
        let clicked_tile = &self.grid.get_tile_by_location(&decision.location);
        // println!("{:?}", clicked_tile);
        match clicked_tile.status {
            TileType::Unopened => self.sweep_grid(&Location {
                x: clicked_tile.location.x,
                y: clicked_tile.location.y,
            }),
            TileType::Mine => return None,
            _ => (),
        }

        if self.grid.amount_unopened_tiles == 0 {
            Some(false)
        } else {
            Some(true)
        }
    }
}

pub struct Decision {
    pub location: Location,
    pub flag: bool,
}

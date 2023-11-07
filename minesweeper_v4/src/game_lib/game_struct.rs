use std::{fmt, ops::Add};

pub struct Grid {
    pub grid: Vec<Tile>,
    pub single_size: usize,
    pub full_size: usize,
    pub amount_unopened_tiles: usize,
}
impl Grid {
    /// <summary>
    /// Create new empty square grid of grid_size.
    /// </summary>
    pub fn new_grid(grid_size: usize) -> Grid {
        let mut grid: Vec<Tile> = Vec::new();
        for x in 0..grid_size {
            for y in 0..grid_size {
                let tile = Tile {
                    location: Location { x, y },
                    status: TileType::Unopened,
                    number: 0,
                    flagged: false,
                };
                grid.push(tile);
            }
        }

        Grid {
            grid,
            single_size: grid_size,
            full_size: grid_size * grid_size,
            amount_unopened_tiles: grid_size * grid_size,
        }
    }

    pub fn location_to_id(&self, location: &Location) -> usize {
        if (location.x < self.single_size) & (location.y < self.single_size) {
            return location.x * self.single_size + location.y;
        }

        panic!(
            "location not on grid. Location ({}, {}). Grid size: {}",
            location.x, location.y, self.single_size
        );
    }

    pub fn id_to_location(&self, id: &usize) -> Location {
        if *id < self.full_size {
            let y_new = id % self.single_size;
            let x_new = (id - y_new) / self.single_size;

            return Location { x: x_new, y: y_new };
        }

        panic!(
            "id out side of grid. id:{}. Grid total size: {}",
            *id, self.full_size
        );
    }

    pub fn get_tile_by_location(&self, location: &Location) -> &Tile {
        &self.grid[self.location_to_id(location)]
    }

    pub fn change_tile_flag_by_location(&mut self, location: Location, new_flag: bool) {
        let id = self.location_to_id(&location);
        self.change_tile_flag_by_id(id, new_flag);
    }

    pub fn change_tile_flag_by_id(&mut self, id: usize, new_flag: bool) {
        if id >= self.full_size {
            panic!(
                "Id provided not on the grid. Id:{}, amount of tiles:{}",
                id, self.full_size
            );
        }
        self.grid[id].flagged = new_flag;
    }

    pub fn change_tile_number_by_id(&mut self, id: usize, new_number: i32) {
        if id >= self.full_size {
            panic!(
                "Id provided not on the grid. Id:{}, amount of tiles:{}",
                id, self.full_size
            );
        }
        self.grid[id].number = new_number;
    }

    pub fn change_tile_status_by_id(&mut self, id: usize, new_status: TileType) {
        if id >= self.full_size {
            panic!(
                "Id provided not on the grid. Id:{}, amount of tiles:{}",
                id, self.full_size
            );
        }

        if self.grid[id].status == TileType::Unopened && new_status != TileType::Unopened {
            self.amount_unopened_tiles -= 1;
        }

        self.grid[id].status = new_status;
    }

    /// <summary>
    /// Returns a vector of all adjacent tile ids. TODO clean up.
    /// </summary>
    pub fn get_neighbours(&self, id: &usize) -> Vec<usize> {
        let mut neighbours: Vec<usize> = Vec::new();

        let mut tile_location_shifted = self.id_to_location(id) + Location { x: 2, y: 2 };
        let mut y_break: usize = 1;

        for dx in 0..3 {
            tile_location_shifted.x = {
                match tile_location_shifted.x.checked_sub(1) {
                    Some(x) => x,
                    None => break,
                }
            };
            for dy in 0..3 {
                y_break = 1;
                tile_location_shifted.y = {
                    match tile_location_shifted.y.checked_sub(1) {
                        Some(y) => y,
                        None => {
                            y_break = 0;
                            continue;
                        }
                    }
                };

                if dx == 1 && dy == 1
                    || tile_location_shifted.x >= self.single_size
                    || tile_location_shifted.y >= self.single_size
                {
                    continue;
                }

                neighbours.push(self.location_to_id(&tile_location_shifted));
            }
            tile_location_shifted.y = tile_location_shifted.y + 2 + y_break;
        }
        neighbours
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Grid:")?;
        writeln!(f, "single_size:{}", self.single_size)?;
        writeln!(f, "full_size:{}", self.full_size)?;
        writeln!(f, "amount_unopened_tiles:{}", self.amount_unopened_tiles)?;

        for x in 0..self.single_size {
            for y in 0..self.single_size {
                let mine_data = {
                    let tile = &self.grid[self.location_to_id(&Location { x, y })];
                    if tile.status == TileType::Mine {
                        "M".to_string()
                    } else {
                        tile.number.to_string()
                    }
                };
                write!(f, " | {} | ", mine_data)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
#[derive(Debug)]
pub struct Tile {
    pub location: Location,
    pub status: TileType,
    pub number: i32,
    pub flagged: bool,
}
#[derive(Debug, PartialEq)]
pub enum TileType {
    Mine,
    Unopened,
    Opened,
}

#[derive(Debug)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl Add<Location> for Location {
    type Output = Location;

    fn add(self, other: Location) -> Location {
        Location {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

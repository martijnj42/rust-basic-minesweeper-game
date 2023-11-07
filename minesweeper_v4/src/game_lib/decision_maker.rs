use super::{
    game_main::Decision,
    game_struct::{Grid, Location, TileType},
};

use rand::Rng;

pub fn random_decision(grid: &Grid) -> Decision {
    let mut rng = rand::thread_rng();

    Decision {
        location: Location {
            x: rng.gen_range(0..grid.single_size),
            y: rng.gen_range(0..grid.single_size),
        },
        flag: false,
    }
}

pub fn logic_with_luck_decision(grid: &Grid) -> Decision {
    for x in 0..grid.single_size {
        for y in 0..grid.single_size {
            let tile = grid.get_tile_by_location(&Location { x, y });

            if tile.status == TileType::Opened && tile.number > 0 {
                let mut unopened_unflagged_neighbours = 0;
                let mut flagged_neighbours = 0;

                let mut last_unopend_tile = Location { x: 0, y: 0 };

                for neighbour in grid.get_neighbours(&grid.location_to_id(&tile.location)) {
                    let neighbour_tile = &grid.grid[neighbour];

                    if neighbour_tile.flagged {
                        flagged_neighbours += 1;
                    } else if (neighbour_tile.status == TileType::Unopened
                        || neighbour_tile.status == TileType::Mine)
                        && !neighbour_tile.flagged
                    {
                        unopened_unflagged_neighbours += 1;

                        last_unopend_tile = Location {
                            x: neighbour_tile.location.x,
                            y: neighbour_tile.location.y,
                        };
                    }
                }

                if tile.number - flagged_neighbours == unopened_unflagged_neighbours
                    && unopened_unflagged_neighbours > 0
                {
                    // flag unopened tile
                    return Decision {
                        location: last_unopend_tile,
                        flag: true,
                    };
                } else if tile.number - flagged_neighbours == 0 && unopened_unflagged_neighbours > 0
                {
                    // click unopened tile
                    return Decision {
                        location: last_unopend_tile,
                        flag: false,
                    };
                }
            }
        }
    }
    random_decision(grid)
}

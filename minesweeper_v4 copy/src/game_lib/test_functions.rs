use std::time::Instant;

use super::{
    game_main::{Decision, Game},
    game_struct::Grid,
};

pub fn time_decision_funcion(
    grid_size: usize,
    mines: usize,
    decision_function: &dyn Fn(&Grid) -> Decision,
    test_iterations: usize,
) {
    let mut game = Game::new_game(grid_size, mines);

    let start_time = Instant::now();

    let mut games_won = 0;
    let mut games_lost = 0;

    for _i in 0..test_iterations {
        game.restart_game();
        loop {
            match game.user_input(decision_function(&game.grid)) {
                Some(game_active) if game_active == false => {
                    games_won = games_won + 1;
                    break;
                }
                None => {
                    games_lost = games_lost + 1;
                    break;
                }
                Some(_) => (),
            }
        }
    }

    let end_time = Instant::now();

    let elapsed_time = end_time.duration_since(start_time);
    println!("Time taken: {} ms", elapsed_time.as_millis());

    println!("Games won: {}", games_won);
    println!("Games lost: {}", games_lost);
}

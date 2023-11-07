// use game_lib::decision_maker;
use game_lib::game_window;
// use game_lib::test_functions;

mod game_lib;

use std::env;

fn main() {
    let mut grid_size = 16;
    let mut mines = 40;

    let max_grid_size = 21;

    // Argument input interpreter
    let args: Vec<String> = env::args().collect();

    // if valid argument for grid size and mines provided update game size
    if args.len() > 2 {
        if let Some(user_grid_input) = args.get(1) {
            if let Ok(new_grid_size) = user_grid_input.parse::<usize>() {
                if new_grid_size > 0 && new_grid_size < max_grid_size {
                    grid_size = new_grid_size;
                } else {
                    println!("Invalid grid size. Please type a number greater than 0 and smaller than 20.");
                    return;
                }
            }
        }

        if let Some(user_mines_input) = args.get(2) {
            if let Ok(new_mines_size) = user_mines_input.parse::<usize>() {
                if new_mines_size > 0 && new_mines_size < grid_size * grid_size {
                    mines = new_mines_size;
                } else {
                    println!("Invalid mine. Please type a number greater than 0 and smaller than the grid size.");
                    return;
                }
            }
        }
    }

    // uncomment below to play the game (normal game size 16 with 40 mines)
    let _ = game_window::start_game_user(grid_size, mines);

    // help function speed and accuracy code

    // let iterations = 10000;
    // println!("Random decision:");
    // test_functions::time_decision_funcion(
    //     grid_size,
    //     mines,
    //     &decision_maker::random_decision,
    //     iterations,
    // );

    // println!("Logic With Luck:");
    // test_functions::time_decision_funcion(
    //     grid_size,
    //     mines,
    //     &decision_maker::logic_with_luck_decision,
    //     iterations,
    // );
}

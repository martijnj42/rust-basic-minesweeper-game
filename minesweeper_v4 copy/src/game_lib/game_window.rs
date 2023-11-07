extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

use super::decision_maker;
use super::event_handeler::EventHandeler;
use super::game_main;
use super::renderer;
use super::visual_settings;

pub fn start_game_user(grid_size: usize, mines: usize) -> Result<(), String> {
    fn check_game_state(game_state: Option<bool>) -> bool {
        match game_state {
            Some(game_active) => {
                if !game_active {
                    println!("You won the game ;)");
                    println!("Press the 'N' key for a new game")
                }
                false
            }
            None => {
                println!("You hit a mine :(");
                println!("Press the 'N' key for a new game");
                true
            }
        }
    }

    let mut game = game_main::Game::new_game(grid_size, mines);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    //change to true to cheat
    let mut developer_mode = false;

    //change to place flags
    let mut flag = false;

    let window = video_subsystem
        .window(
            "Mine Sweeper",
            (game.grid.single_size * visual_settings::PIXELS_PER_TILE) as u32,
            (game.grid.single_size * visual_settings::PIXELS_PER_TILE) as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    // start renderer here
    let mut renderer = renderer::Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => developer_mode = !developer_mode,

                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => flag = !flag,

                Event::KeyDown {
                    keycode: Some(Keycode::N),
                    ..
                } => {
                    developer_mode = false;
                    flag = false;
                    game.restart_game()
                }

                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    if check_game_state(
                        game.user_input(decision_maker::logic_with_luck_decision(&game.grid)),
                    ) {
                        developer_mode = true;
                    }
                }

                Event::MouseButtonDown { x, y, .. } => {
                    if let Ok(tile_location) = EventHandeler::calculate_tile_clicked(
                        game.grid.single_size,
                        x as usize,
                        y as usize,
                    ) {
                        if check_game_state(game.user_input(game_main::Decision {
                            location: tile_location,
                            flag,
                        })) {
                            developer_mode = true;
                        }
                    }
                }
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...

        // draw game here
        renderer.draw_grid(&game.grid, developer_mode)?;
    }

    Ok(())
}

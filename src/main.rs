#[macro_use]
extern crate stdweb;

mod game;
mod utils;

fn main() {
    stdweb::initialize();

    let game = game::Game::new("#canvas", 800., 600.);

    fill_random_grid(&game);

    stdweb::event_loop();
}

fn fill_random_grid(game: &game::Game) {
    for x in 0..game.grid_width {
        for y in 0..game.grid_height {
            game.draw_cell(x, y, &utils::get_random_color());
        }
    }
}

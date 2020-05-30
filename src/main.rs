#[macro_use]
extern crate stdweb;
use stdweb::unstable::TryInto;

mod game;

fn main() {
    stdweb::initialize();

    let game = game::Game::new("#canvas", 800., 600.);

    for x in 0..40 {
        for y in 0..30 {
            game.draw_cell(x, y, &get_random_color());
        }
    }

    stdweb::event_loop();
}

fn get_random_color() -> String {
    let letters = "01234567890ABCDEF";
    let len = letters.len() as u32;
    let mut color: String = String::from("#");
    for _ in 0..6 {
        let random_value: u32 = js! {return Math.floor(Math.random() * @{len})}
            .try_into()
            .unwrap();
        color.push(letters.chars().nth(random_value as usize).unwrap());
    }
    color
}

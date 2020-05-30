use crate::game;
use stdweb::unstable::TryInto;

#[allow(dead_code)]
pub fn fill_random_grid(game: &game::Game) {
    for x in 0..game.grid_width {
        for y in 0..game.grid_height {
            game.draw_cell(x, y, &get_random_color());
        }
    }
}

fn get_random_color() -> String {
    let letters = "01234567890ABCDEF";
    let len = letters.len() as u32;
    let mut color: String = String::from("#");
    for _ in 0..6 {
        let random_value: usize = js! {return Math.floor(Math.random() * @{len})}
            .try_into()
            .unwrap();
        color.push(letters.chars().nth(random_value).unwrap());
    }
    color
}

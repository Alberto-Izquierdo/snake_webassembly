#[macro_use]
extern crate stdweb;

use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::web::{event::KeyDownEvent, IEventTarget};

mod game;
mod snake;
mod utils;

fn main() {
    stdweb::initialize();

    let game = game::Game::new("#canvas", 800., 600.);

    let snake = Rc::new(RefCell::new(snake::Snake::new(&game)));

    stdweb::web::document().add_event_listener({
        let snake = snake.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => snake.borrow_mut().change_direction(snake::Direction::LEFT),
                "ArrowRight" => snake.borrow_mut().change_direction(snake::Direction::RIGHT),
                "ArrowDown" => snake.borrow_mut().change_direction(snake::Direction::DOWN),
                "ArrowUp" => snake.borrow_mut().change_direction(snake::Direction::UP),
                _ => {}
            };
        }
    });

    fn game_loop(snake: Rc<RefCell<snake::Snake>>, game: Rc<game::Game>, time: u32) {
        stdweb::web::set_timeout(
            move || {
                game_loop(snake.clone(), game.clone(), time);
                snake.borrow_mut().update(&game);
            },
            time,
        );
    }

    game_loop(snake, Rc::new(game), 100);

    //fill_random_grid(&game);

    stdweb::event_loop();
}

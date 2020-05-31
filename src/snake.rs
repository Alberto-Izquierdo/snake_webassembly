use crate::game;
use crate::utils;

#[derive(Copy, Clone, PartialEq)]
struct Cell(u32, u32);

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn is_opposite(self, other: Direction) -> bool {
        self == Direction::LEFT && other == Direction::RIGHT
            || self == Direction::RIGHT && other == Direction::LEFT
            || self == Direction::UP && other == Direction::DOWN
            || self == Direction::DOWN && other == Direction::UP
    }
}

pub struct Snake {
    body: Vec<Cell>,
    current_direction: Option<Direction>,
    previous_direction: Direction,
    food: Cell,
}

impl Snake {
    pub fn new(game: &game::Game) -> Snake {
        let body = vec![Cell(
            utils::generate_random_number(game.grid_width),
            utils::generate_random_number(game.grid_height),
        )];
        let previous_direction = Direction::RIGHT;
        let current_direction = Some(Direction::RIGHT);
        let food = spawn_random_food(&body, game);
        Snake {
            body,
            current_direction,
            previous_direction,
            food,
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if !self.previous_direction.is_opposite(new_direction) {
            self.current_direction = Some(new_direction);
        }
    }

    fn get_next_position(cell: Cell, direction: Direction, game: &game::Game) -> Cell {
        match direction {
            Direction::RIGHT => Cell((cell.0 + 1) % game.grid_width, cell.1),
            Direction::LEFT => Cell(cell.0.checked_sub(1).unwrap_or(game.grid_width - 1), cell.1),
            Direction::DOWN => Cell(cell.0, (cell.1 + 1) % game.grid_height),
            Direction::UP => Cell(
                cell.0,
                cell.1.checked_sub(1).unwrap_or(game.grid_height - 1),
            ),
        }
    }

    pub fn update(&mut self, game: &game::Game) {
        let direction = self.current_direction.unwrap_or(self.previous_direction);

        let head = self.body.first().cloned().unwrap();

        self.body
            .insert(0, Snake::get_next_position(head, direction, game));

        self.body.pop();

        let head = self.body.first().cloned().unwrap();

        if head == self.food {
            self.body.push(self.food);
            self.food = spawn_random_food(&self.body, game);
        } else {
            for (_, cell) in self.body.iter().enumerate().skip(1) {
                if *cell == head {
                    self.reset(game);
                    return;
                }
            }
        }

        self.previous_direction = direction;
        self.draw(game);
    }

    fn draw(&self, game: &game::Game) {
        game.clear();
        let head = self.body.first().cloned().unwrap();
        for (_, cell) in self.body.iter().enumerate().skip(1) {
            game.draw_cell(cell.0, cell.1, "green");
        }
        game.draw_cell(head.0, head.1, "lightgreen");
        game.draw_cell(self.food.0, self.food.1, "red");
    }

    fn reset(&mut self, game: &game::Game) {
        self.body = vec![Cell(
            utils::generate_random_number(game.grid_width),
            utils::generate_random_number(game.grid_height),
        )];
        self.food = spawn_random_food(&self.body, game);
        self.previous_direction = Direction::RIGHT;
        self.current_direction = None;
    }
}

fn spawn_random_food(snake: &Vec<Cell>, game: &game::Game) -> Cell {
    let mut result: Option<Cell> = None;
    while result.is_none() {
        let x = utils::generate_random_number(game.grid_width);
        let y = utils::generate_random_number(game.grid_height);
        let food = Cell(x, y);
        let res: Vec<&Cell> = snake.into_iter().filter(|cell| **cell == food).collect();
        if res.is_empty() {
            result = Some(food)
        }
    }
    result.unwrap()
}

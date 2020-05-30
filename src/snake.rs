use crate::game;

#[derive(Copy, Clone)]
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
}

impl Snake {
    pub fn new() -> Snake {
        let body = vec![Cell(0, 0), Cell(0, 0), Cell(0, 0), Cell(0, 0), Cell(0, 0)];
        let previous_direction = Direction::RIGHT;
        let current_direction = Some(Direction::RIGHT);
        Snake {
            body,
            current_direction,
            previous_direction,
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

        self.previous_direction = direction;
        self.draw(game);
    }

    pub fn draw(&self, game: &game::Game) {
        game.clear();
        for cell in &self.body {
            game.draw_cell(cell.0, cell.1, "green");
        }
    }
}

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct Game {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    pub grid_width: u32,
    pub grid_height: u32,
    width: f64,
    height: f64,
    cell_width: f64,
    cell_height: f64,
}

impl Game {
    pub fn new(canvas_id: &str, width: f64, height: f64) -> Game {
        let canvas: CanvasElement = document()
            .query_selector(canvas_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
        let grid_width = 20;
        let grid_height = 15;
        let cell_width: f64 = width / grid_width as f64;
        let cell_height: f64 = height / grid_height as f64;
        Game {
            canvas,
            ctx,
            width,
            height,
            grid_width,
            grid_height,
            cell_width,
            cell_height,
        }
    }

    pub fn draw_cell(&self, x: u32, y: u32, color: &str) {
        assert!(
            x < self.grid_width as u32,
            "X position higher than the grid's width"
        );
        assert!(
            y < self.grid_height as u32,
            "Y position higher than the grid's height"
        );
        self.ctx.set_fill_style_color(color);
        self.ctx.fill_rect(
            x as f64 * self.width / self.grid_width as f64,
            y as f64 * self.height / self.grid_height as f64,
            self.cell_width,
            self.cell_height,
        )
    }

    pub fn clear(&self) {
        self.ctx.set_fill_style_color("white");
        self.ctx.fill_rect(0.0, 0.0, self.width, self.height);
    }
}

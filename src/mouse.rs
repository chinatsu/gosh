use ggez::graphics::{self, Canvas, Rect};
use crate::{Grid, Point};

#[derive(Clone, Debug)]
pub struct Mouse {
    pub possible_moves: Vec<(Point, u32)>,
    size: usize,
    x: usize,
    y: usize,
    max_moves: u32,
}

impl Mouse {
    pub fn new(size: u32, max_moves: u32) -> Mouse {
        Mouse {
            max_moves: max_moves,
            possible_moves: Vec::new(),
            size: size as usize,
            x: 0,
            y: 0,
        }
    }

    pub fn update(&mut self, grid: &Grid) {
        if !grid.within_bounds(self.x, self.y) {
            return
        }
        self.possible_moves = Vec::new();
        let mut possible_moves = vec![(Point{x: self.x, y: self.y}, 0)];

        while possible_moves.len() != 0 {
            if let Some((point, distance)) = possible_moves.pop() {
                if grid.tile_at(point.x, point.y) {
                    if distance <= self.max_moves {
                        for neighbor in grid.neighbors_at(point.x, point.y) {
                            possible_moves.push((neighbor, distance+1));
                            self.possible_moves.push((neighbor, distance+1));
                        }
                    }
                }
            }
        }
    }

    pub fn update_position(&mut self, x: f32, y: f32) {
        self.x = (x/self.size as f32) as usize;
        self.y = (y/self.size as f32) as usize;
    }

    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for (possible_move, _) in &self.possible_moves {
            println!("{possible_move:?}");
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(Rect{x: (possible_move.x*self.size) as f32, y: (possible_move.y*self.size) as f32, w: self.size as f32, h: self.size as f32})
                    .color([0.0, 1.0, 0.0, 1.0]),
            );
        }
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(Rect{x: (self.x*self.size) as f32, y: (self.y*self.size) as f32, w: self.size as f32, h: self.size as f32})
                .color([1.0, 0.5, 0.0, 1.0]),
        );
    }
}
use ggez::graphics::{self, Canvas, Rect};
use crate::{Grid, Point};

#[derive(Clone, Debug)]
pub struct Mouse {
    pub possible_moves: Vec<Point>,
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

    pub fn find_movement_area(&mut self, grid: &Grid, x: usize, y: usize, depth: u32) {
        let point = Point{x: x, y: y};
        #[cfg(feature = "possible-moves-contains")]
        {
            if !self.possible_moves.contains(&point) && depth <= self.max_moves {
                self.possible_moves.push(point);
                for neighbor in grid.neighbors_at(point.x, point.y) {
                    self.find_movement_area(grid, neighbor.x, neighbor.y, depth+1);
                }
            }
        }
        #[cfg(not(feature = "possible-moves-contains"))]
        {
            if depth <= self.max_moves {
                self.possible_moves.push(point);
                for neighbor in grid.neighbors_at(point.x, point.y) {
                    self.find_movement_area(grid, neighbor.x, neighbor.y, depth+1);
                }
            }
        }
        
    }
    pub fn update(&mut self, grid: &Grid) {
        self.possible_moves = Vec::new();
        self.find_movement_area(grid, self.x, self.y, 0);
    }

    pub fn update_position(&mut self, x: f32, y: f32) {
        self.x = (x/self.size as f32) as usize;
        self.y = (y/self.size as f32) as usize;
    }

    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for possible_move in &self.possible_moves {
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
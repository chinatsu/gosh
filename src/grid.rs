use ggez::{Context, GameResult};
use ggez::graphics::{self, MeshBuilder, Canvas, Mesh, Rect};
use ggez::mint::Point2;
use crate::Component;

#[derive(Debug)]
pub struct Grid {
    mesh: Mesh,
    field: Vec<Vec<bool>>,
    size: u32,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Grid {
    pub fn new(ctx: &mut Context, size: u32) -> GameResult<Grid> {
        let (w, h) = ctx.gfx.drawable_size();
        let mut mesh = MeshBuilder::new();
        for x in 0..w as u32/size {
            mesh.line(&[Point2{x: (x*size) as f32, y: 0.0}, Point2{x: (x*size) as f32, y: h}], 2.0, graphics::Color::BLACK)?;
        }
        for y in 0..h as u32/size {
            mesh.line(&[Point2{x: 0.0, y: (y*size) as f32}, Point2{x: w, y: (y*size) as f32}], 2.0, graphics::Color::BLACK)?;
        }

        Ok(Grid {
            mesh: Mesh::from_data(ctx, mesh.build()),
            field: vec![vec![false; h as usize/size as usize]; w as usize/size as usize],
            size: size,
        })
    }

    pub fn toggle_position(&mut self, x: usize, y: usize) {
        self.field[x][y] = !self.field[x][y];
    }

    pub fn neighbors_at(&self, x: usize, y: usize) -> Vec<Point> {
        let mut neighbors: Vec<Point> = Vec::new();
        if x >= self.field.len() || y >= self.field[0].len() {
            return neighbors
        }
        for delta in vec![1, -1] {
            let new_x = x as isize + delta;
            let new_y = y as isize + delta;
            if new_x >= 0 && new_x < self.field.len() as isize {
                let node = self.field[new_x as usize][y];
                if !node {
                    neighbors.push(Point{x: new_x as usize, y: y});
                }
            }
            if new_y >= 0 && new_y < self.field[0].len() as isize {
                let node = self.field[x][new_y as usize];
                if !node {
                    neighbors.push(Point{x: x, y: new_y as usize});
                }
            }
        }
        neighbors
    }
}

impl Component for Grid {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, canvas: &mut Canvas) {
        for (x, line) in self.field.iter().enumerate() {
            for (y, element) in line.iter().enumerate() {
                if *element {
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(Rect{x: (x as u32*self.size) as f32, y: (y as u32*self.size) as f32, w: self.size as f32, h: self.size as f32})
                            .color([0.0, 0.0, 1.0, 1.0]),
                    );
                }
            }
        }
        canvas.draw(&self.mesh, graphics::DrawParam::new().image_scale(false));
    }
}
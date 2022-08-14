use ggez::{Context, GameResult};
use ggez::graphics::{self, MeshBuilder, Canvas, Mesh};
use ggez::mint::Point2;
use crate::Component;

pub struct Grid {
    mesh: Mesh,
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
            mesh: Mesh::from_data(ctx, mesh.build())
        })
    }
}

impl Component for Grid {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.draw(&self.mesh, graphics::DrawParam::new().image_scale(false));
    }
}
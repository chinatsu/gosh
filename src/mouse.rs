use ggez::graphics::{self, Canvas, Rect};

#[derive(Copy, Clone)]
pub struct Mouse {
    size: u32,
    x: u32,
    y: u32,
}

impl Mouse {
    pub fn new(size: u32) -> Mouse {
        Mouse {
            size: size,
            x: 0,
            y: 0,
        }
    }

    pub fn update_position(&mut self, x: f32, y: f32) {
        self.x = (x/self.size as f32) as u32;
        self.y = (y/self.size as f32) as u32;
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(Rect{x: (self.x*self.size) as f32, y: (self.y*self.size) as f32, w: self.size as f32, h: self.size as f32})
                .color([1.0, 0.5, 0.0, 1.0]),
        );
    }
}
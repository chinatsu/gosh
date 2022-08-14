use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Canvas};
use ggez::event::{self, EventHandler};
use ggez::conf::WindowMode;

mod grid;
mod mouse;
use mouse::Mouse;

pub trait Component {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn draw(&mut self, canvas: &mut Canvas);
}

fn main() -> GameResult<()> {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("gosh", "cn")
        .window_setup(ggez::conf::WindowSetup::default().title("Gosh"))
        .window_mode(WindowMode::default().dimensions(640.0, 480.0))
        .build()
        .expect("could not create ggez context!");

    let grid_size = 32;

    let my_game = Gosh::new(&mut ctx)
        .with_component(grid::Grid::new(&mut ctx, grid_size)?)
        .with_mouse(Mouse::new(grid_size));

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct Gosh {
    components: Vec<Box<dyn Component + 'static>>,
    mouse: Option<mouse::Mouse>,
}

impl Gosh {
    pub fn new(_ctx: &mut Context) -> Gosh {
        Gosh {
            components: Vec::new(),
            mouse: None
        }
    }

    pub fn with_mouse(mut self, mouse: Mouse) -> Gosh {
        if self.mouse.is_none() {
            self.mouse = Some(mouse);
        } else {
            panic!("You can't call .with_mouse() more than once!");
        }
        self
    }

    pub fn with_component(mut self, component: impl Component + 'static) -> Gosh {
        self.components.push(Box::new(component));
        self
    }
}

impl EventHandler for Gosh {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for component in &mut self.components {
            // no component does anything sensible at the moment
            component.update(ctx)?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear(Color::WHITE),
        );
        if let Some(mouse) = self.mouse {
            mouse.draw(&mut canvas);
        }
        for component in &mut self.components {
            component.draw(&mut canvas);
        }
        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _xrel: f32,
        _yrel: f32,
    ) -> GameResult {
        if let Some(mouse) = &mut self.mouse {
            mouse.update_position(x, y);
        }
        Ok(())
    }
}
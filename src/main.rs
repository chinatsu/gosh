use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Canvas};
use ggez::event::{self, EventHandler};
use ggez::input::mouse::MouseButton;
use ggez::conf::{WindowMode, WindowSetup};

mod grid;
use grid::{Grid, Point};
mod mouse;
use mouse::Mouse;

pub trait Component {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn draw(&mut self, canvas: &mut Canvas);
}

const DESIRED_FPS: u32 = 60;
const GRID_SIZE: u32 = 32;
const MAX_MOVES: u32 = 5;

fn main() -> GameResult<()> {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("gosh", "cn")
        .window_setup(WindowSetup::default().title("Gosh"))
        .window_mode(WindowMode::default().dimensions(640.0, 480.0))
        .build()
        .expect("could not create ggez context!");

    let my_game = Gosh::new(&mut ctx)
        .with_grid(grid::Grid::new(&mut ctx, GRID_SIZE)?)
        .with_mouse(Mouse::new(GRID_SIZE, MAX_MOVES));

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct Gosh {
    components: Vec<Box<dyn Component + 'static>>,
    mouse: Option<mouse::Mouse>,
    grid: Option<grid::Grid>,
}

impl Gosh {
    pub fn new(_ctx: &mut Context) -> Self {
        Gosh {
            components: Vec::new(),
            grid: None,
            mouse: None,
        }
    }

    pub fn with_mouse(mut self, mouse: Mouse) -> Self {
        if self.mouse.is_none() {
            self.mouse = Some(mouse);
        } else {
            panic!("You can't call .with_mouse() more than once!");
        }
        self
    }

    pub fn with_grid(mut self, grid: Grid) -> Self {
        if self.grid.is_none() {
            self.grid = Some(grid);
        } else {
            panic!("You can't call .with_grid() more than once!");
        }
        self
    }

    pub fn with_component(mut self, component: impl Component + 'static) -> Self {
        self.components.push(Box::new(component));
        self
    }
}

impl EventHandler for Gosh {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ctx.time.check_update_time(DESIRED_FPS) {
            for component in &mut self.components {
                // no component does anything sensible at the moment
                component.update(ctx)?;
            }
            if let Some(grid) = &mut self.grid {
                if let Some(mouse) = &mut self.mouse {
                    mouse.update(grid);
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear(Color::WHITE),
        );
        if let Some(mouse) = &self.mouse {
            mouse.draw(&mut canvas);
        }
        for component in &mut self.components {
            component.draw(&mut canvas);
        }
        if let Some(grid) = &mut self.grid {
            grid.draw(&mut canvas);
        }
        canvas.finish(ctx)?;
        ggez::timer::yield_now();
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

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        if let Some(grid) = &mut self.grid {
            if let Some(mouse) = &mut self.mouse {
                let (x, y) = mouse.position();
                match button {
                    MouseButton::Right => grid.toggle_position(x, y),
                    _ => ()
                }
            }
        }

        Ok(())
        
    }
}
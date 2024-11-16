use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::EventHandler;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::conf::{WindowMode, WindowSetup};
use glam::Vec2;

mod terrain;
use terrain::Terrain;

// Constants from the Processing version
const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 640.0;
const ORIGINAL_WIDTH: f32 = 480.0;
const ORIGINAL_HEIGHT: f32 = 480.0;
const SCALE_X: f32 = WINDOW_WIDTH / ORIGINAL_WIDTH;
const SCALE_Y: f32 = WINDOW_HEIGHT / ORIGINAL_HEIGHT;

struct GameState {
    state: i32,  // 0 = title, 1 = playing, 2 = game over
    score: i32,
    speed_x: f32,
    speed_x_delta: f32,
    speed_y: f32,
    gravity: f32,
    ascent_speed: f32,
    collision_counter_on: bool,
    title_text: graphics::Image,
    terrain: Terrain,
    mouse_pos: Vec2,
}

impl GameState {
    fn new(ctx: &Context) -> GameResult<GameState> {
        Ok(GameState {
            state: 0,
            score: 0,
            speed_x: 4.0 * SCALE_X,
            speed_x_delta: 4.0 * SCALE_X,
            speed_y: 1.0 * SCALE_Y,
            gravity: 0.4 * SCALE_Y,
            ascent_speed: 0.0,
            collision_counter_on: false,
            title_text: graphics::Image::from_path(ctx, "/resources/TitleText.png")?,
            terrain: Terrain::new(
                WINDOW_WIDTH,
                WINDOW_HEIGHT - WINDOW_HEIGHT/6.0,
                4.0 * SCALE_X
            ),
            mouse_pos: Vec2::new(0.0, WINDOW_HEIGHT/6.0),
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        match self.state {
            0 => {}, // Title screen
            1 => {   // Playing
                self.terrain.update(
                    self.mouse_pos,
                    self.speed_x,
                    self.ascent_speed,
                    WINDOW_WIDTH
                );
            },
            2 => {}, // Game over
            _ => {},
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx, 
            Color::from_rgb(0, 0, 40) // #000028
        );

        match self.state {
            0 => { // Title screen
                let params = graphics::DrawParam::default()
                    .dest(Vec2::new(0.0, WINDOW_HEIGHT/4.0))
                    .scale(Vec2::new(
                        WINDOW_WIDTH/self.title_text.width() as f32,
                        SCALE_Y
                    ));
                canvas.draw(&self.title_text, params);
            },
            1 => { // Playing
                self.terrain.draw(ctx, &mut canvas, WINDOW_HEIGHT)?;
            },
            2 => { // Game over
                // Draw game over screen
            },
            _ => {},
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> GameResult {
        self.mouse_pos = Vec2::new(x, y);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        match input.keycode {
            Some(KeyCode::Space) => {
                if self.state == 0 {
                    self.state = 1;
                }
            },
            Some(KeyCode::A) => {
                self.speed_x += 0.2;
            },
            Some(KeyCode::S) => {
                self.speed_x -= 0.2;
                if self.speed_x < 0.0 {
                    self.speed_x = 0.0;
                }
            },
            Some(KeyCode::R) => {
                self.terrain.realign();
            },
            Some(KeyCode::Key0) => self.state = 0,
            Some(KeyCode::Key1) => self.state = 1,
            _ => (),
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        if self.state == 0 {
            self.state = 1;
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("high-mountain-trucker", "you")
        .window_setup(WindowSetup::default().title("High Mountain Trucker"))
        .window_mode(WindowMode::default()
            .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));
    
    let (mut ctx, event_loop) = cb.build()?;
    let mut state = GameState::new(&ctx)?;
    event_loop.run(move |_event, _window_target, control_flow| {
        *control_flow = ggez::event::ControlFlow::Poll;
        if let Err(e) = state.update(&mut ctx) {
            println!("Error updating game: {}", e);
            *control_flow = ggez::event::ControlFlow::Exit;
        }
        if let Err(e) = state.draw(&mut ctx) {
            println!("Error drawing game: {}", e);
            *control_flow = ggez::event::ControlFlow::Exit;
        }
    })
}

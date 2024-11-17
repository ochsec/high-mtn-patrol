use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam};
use ggez::event::EventHandler;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::conf::{WindowMode, WindowSetup};
use glam::Vec2;

mod terrain;
mod player;
mod background;
mod boulder;
mod pickup;
use terrain::Terrain;
use player::Player;
use background::Background;
use boulder::Boulder;
use pickup::{Pickup, PickupType};

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
    background: Background,
    mouse_pos: Vec2,
    player: Player,
    boulders: Vec<Boulder>,
    pickups: Vec<Pickup>,
}

impl GameState {
    fn new(ctx: &Context) -> GameResult<GameState> {
        let mut state = GameState {
            state: 0,
            score: 0,
            speed_x: 4.0 * SCALE_X,
            speed_x_delta: 4.0 * SCALE_X,
            speed_y: 1.0 * SCALE_Y,
            gravity: 15.0 * SCALE_Y,
            ascent_speed: 0.0,
            collision_counter_on: false,
            title_text: graphics::Image::from_path(ctx, "/TitleText.png")?,
            terrain: Terrain::new(
                WINDOW_WIDTH,
                WINDOW_HEIGHT - WINDOW_HEIGHT/6.0,
                4.0 * SCALE_X
            ),
            background: Background::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            mouse_pos: Vec2::new(0.0, WINDOW_HEIGHT/6.0),
            player: Player::new(WINDOW_WIDTH/4.0, WINDOW_HEIGHT/2.0),
            boulders: Vec::new(),
            pickups: Vec::new(),
        };
        
        state.reset_game_environment(ctx)?;
        Ok(state)
    }

    fn reset_game_environment(&mut self, _ctx: &Context) -> GameResult {
        // Reset boulders
        self.boulders.clear();
        for _ in 0..2 {
            self.boulders.push(Boulder::new(
                WINDOW_WIDTH,
                rand::random::<f32>() * WINDOW_HEIGHT * 0.75,
                WINDOW_WIDTH
            ));
        }

        // Reset pickups
        self.pickups.clear();
        // Add coins
        for i in 0..2 {
            self.pickups.push(Pickup::new(
                WINDOW_WIDTH + i as f32 * 200.0,
                rand::random::<f32>() * WINDOW_HEIGHT,
                PickupType::Coin
            ));
        }
        // Add gems
        for i in 0..2 {
            self.pickups.push(Pickup::new(
                WINDOW_WIDTH + i as f32 * 300.0,
                rand::random::<f32>() * WINDOW_HEIGHT,
                PickupType::Gem
            ));
        }

        // Reset player position
        self.player = Player::new(WINDOW_WIDTH/4.0, WINDOW_HEIGHT/2.0);

        // Reset terrain
        self.terrain = Terrain::new(
            WINDOW_WIDTH,
            WINDOW_HEIGHT - WINDOW_HEIGHT/6.0,
            4.0 * SCALE_X
        );

        // Reset game state variables
        self.score = 0;
        self.speed_x = 4.0 * SCALE_X;
        self.speed_x_delta = 4.0 * SCALE_X;
        self.speed_y = 1.0 * SCALE_Y;
        self.gravity = 15.0 * SCALE_Y;
        self.ascent_speed = 0.0;
        self.collision_counter_on = false;

        Ok(())
    }
}


impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            0 => {}, // Title screen
            1 => {   // Playing
                // Update terrain
                self.terrain.update(
                    self.mouse_pos,
                    self.speed_x,
                    self.ascent_speed,
                    &mut self.player
                );

                // Update player with gravity and terrain physics
                self.player.update(ctx.time.delta().as_secs_f32(), self.gravity, &mut self.speed_x);

                // Check for keyboard input
                let keyboard = &ctx.keyboard;
                
                if keyboard.is_key_pressed(KeyCode::Space) {
                    self.player.jump();
                }
                
                if keyboard.is_key_pressed(KeyCode::Left) {
                    self.player.move_left();
                } else if keyboard.is_key_pressed(KeyCode::Right) {
                    self.player.move_right();
                } else {
                    self.player.stop_horizontal();
                }

                // Check collisions
                if self.player.pos.y > WINDOW_HEIGHT {
                    self.collision_counter_on = true;
                    self.state = 2;  // Game over
                } else if let Some(terrain_height) = self.terrain.get_height_at(self.player.pos.x) {
                    // Add a small buffer to prevent premature ground detection
                    if self.player.pos.y + 30.0 > terrain_height - 5.0 {  
                        self.player.pos.y = terrain_height - 30.0;  // Align bottom of wheels with terrain
                        self.player.velocity.y = 0.0;
                        self.player.on_ground = true;
                    } else {
                        self.player.on_ground = false;
                    }
                }
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
                self.background.update(self.speed_x, self.ascent_speed);
                self.background.draw(ctx, &mut canvas)?;
                self.terrain.draw(ctx, &mut canvas, WINDOW_HEIGHT)?;

                // Draw score
                let score_text = format!("Score: {}", self.score);
                let score_display = graphics::Text::new(score_text);
                canvas.draw(&score_display, DrawParam::default()
                    .dest(Vec2::new(20.0, 20.0))
                    .color(Color::RED));

                // Update and draw boulders
                for boulder in &mut self.boulders {
                    boulder.update(self.speed_x, self.ascent_speed);
                    
                    // Check if boulder is off screen
                    if boulder.pos.x + boulder.width < 0.0 {
                        boulder.reposition(WINDOW_WIDTH, WINDOW_HEIGHT);
                    }
                    
                    // Check collision with player
                    if boulder.collides_with_player(self.player.pos, self.player.width, self.player.height) {
                        self.collision_counter_on = true;
                        self.state = 2; // Game over
                    }
                    
                    boulder.draw(ctx, &mut canvas)?;
                }

                // Update and draw pickups
                for pickup in &mut self.pickups {
                    pickup.update(self.speed_x, self.ascent_speed);
                    
                    // Check if pickup is off screen
                    if pickup.pos.x + pickup.width < 0.0 {
                        pickup.reposition(WINDOW_WIDTH, WINDOW_HEIGHT);
                    }
                    
                    // Check collision with player
                    if pickup.collides_with_player(self.player.pos, self.player.width, self.player.height) {
                        pickup.collected = true;
                        self.score += pickup.value;
                        self.speed_x += 5.0 * SCALE_X;
                    }
                    
                    pickup.draw(ctx, &mut canvas)?;
                }

                // Only draw player if not collided
                if !self.collision_counter_on {
                    self.player.draw(ctx, &mut canvas)?;
                }
            },
            2 => { // Game over
                self.background.draw(ctx, &mut canvas)?;
                self.terrain.draw(ctx, &mut canvas, WINDOW_HEIGHT)?;
                
                // Draw game over text
                let game_over = graphics::Text::new("Game Over");
                let score_text = graphics::Text::new(format!("Final Score: {}", self.score));
                
                canvas.draw(&game_over, DrawParam::default()
                    .dest(Vec2::new(WINDOW_WIDTH/2.0 - 100.0, WINDOW_HEIGHT/3.0))
                    .color(Color::RED));
                    
                canvas.draw(&score_text, DrawParam::default()
                    .dest(Vec2::new(WINDOW_WIDTH/2.0 - 80.0, WINDOW_HEIGHT/2.0))
                    .color(Color::WHITE));
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
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        match input.keycode {
            Some(KeyCode::Return) => {
                match self.state {
                    0 => {
                        // Start new game
                        self.state = 1;
                        self.reset_game_environment(ctx)?;
                    },
                    2 => {
                        // Return to title screen from game over
                        self.state = 0;
                    },
                    _ => {},
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
        .add_resource_path("resources")  // Add this line
        .window_setup(WindowSetup::default().title("High Mountain Trucker"))
        .window_mode(WindowMode::default()
            .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));
    
    let (ctx, event_loop) = cb.build()?;
    let state = GameState::new(&ctx)?;
    ggez::event::run(ctx, event_loop, state)
}

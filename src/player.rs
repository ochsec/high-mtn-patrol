use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Rect};
use glam::Vec2;

pub struct Player {
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
    pub velocity: Vec2,
    pub on_ground: bool,
    pub jump_speed: f32,
    pub move_speed: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            pos: Vec2::new(x, y),
            width: 30.0,
            height: 30.0,
            velocity: Vec2::ZERO,
            on_ground: false,
            jump_speed: -400.0,
            move_speed: 200.0,
        }
    }

    pub fn update(&mut self, dt: f32, gravity: f32) {
        // Apply gravity
        if !self.on_ground {
            self.velocity.y += gravity;
        }
        
        // Update position
        self.pos += self.velocity * dt;
    }

    pub fn jump(&mut self) {
        if self.on_ground {
            self.velocity.y = self.jump_speed;
            self.on_ground = false;
        }
    }

    pub fn move_left(&mut self) {
        self.velocity.x = -self.move_speed;
    }

    pub fn move_right(&mut self) {
        self.velocity.x = self.move_speed;
    }

    pub fn stop_horizontal(&mut self) {
        self.velocity.x = 0.0;
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let player_rect = Rect::new(
            self.pos.x - self.width/2.0,
            self.pos.y - self.height/2.0,
            self.width,
            self.height
        );

        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            player_rect,
            Color::from_rgb(255, 0, 0), // Red color for player
        )?;
        canvas.draw(&mesh, DrawParam::default());
        Ok(())
    }
}

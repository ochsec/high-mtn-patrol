use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Rect};
use glam::Vec2;

pub struct Player {
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
    pub velocity: Vec2,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            pos: Vec2::new(x, y),
            width: 30.0,
            height: 30.0,
            velocity: Vec2::ZERO,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos += self.velocity * dt;
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
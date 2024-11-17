use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Rect, Mesh, Canvas};
use ggez::glam::Vec2;
use rand::Rng;

pub struct Boulder {
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Boulder {
    pub fn new(x: f32, y: f32, window_width: f32) -> Self {
        let mut rng = rand::thread_rng();
        let width = rng.gen_range(window_width/20.0..6.0*window_width/20.0);
        let height = rng.gen_range(window_width/20.0..6.0*window_width/20.0);

        Boulder {
            pos: Vec2::new(x, y),
            width,
            height,
            color: Color::from_rgb(100, 100, 100),
        }
    }

    pub fn update(&mut self, speed_x: f32, ascent: f32) {
        self.pos.x -= speed_x;
        self.pos.y += ascent;
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let boulder_rect = Rect::new(
            self.pos.x,
            self.pos.y,
            self.width,
            self.height
        );

        let mesh = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            boulder_rect,
            self.color,
        )?;
        canvas.draw(&mesh, DrawParam::default());
        Ok(())
    }

    pub fn reposition(&mut self, window_width: f32, window_height: f32) {
        let mut rng = rand::thread_rng();
        self.pos.x = window_width;
        self.pos.y = rng.gen_range(-10.0..3.0*window_height/4.0);
        self.width = rng.gen_range(window_width/20.0..6.0*window_width/20.0);
        self.height = rng.gen_range(window_width/20.0..6.0*window_width/20.0);
    }

    pub fn collides_with_player(&self, player_pos: Vec2, player_width: f32, player_height: f32) -> bool {
        let player_left = player_pos.x;
        let player_right = player_pos.x + player_width;
        let player_top = player_pos.y - player_height;
        let player_bottom = player_pos.y;
        
        let boulder_left = self.pos.x;
        let boulder_right = self.pos.x + self.width;
        let boulder_top = self.pos.y;
        let boulder_bottom = self.pos.y + self.height;

        player_left < boulder_right && 
        player_right > boulder_left &&
        player_top < boulder_bottom && 
        player_bottom > boulder_top
    }
}

use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Rect};
use ggez::glam::Vec2;
use crate::player::Player;
use crate::WINDOW_WIDTH;

pub struct Bar {
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
    speed: f32,
    ascent: f32,
}

impl Bar {
    pub fn new(x: f32, y: f32, speed: f32, width: f32) -> Self {
        Bar {
            pos: Vec2::new(x, y),
            width,
            height: 0.0,
            speed,
            ascent: 0.0,
        }
    }

    pub fn update(&mut self, mouse_pos: Vec2, speed_x: f32, ascent: f32, player: &mut Player) {
        self.speed = speed_x;
        self.ascent = ascent;
        self.pos.x -= self.speed;
        self.pos.y += self.ascent;

        // Adjust height based on mouse position
        let half_width = self.width / 2.0;
        if self.pos.x > mouse_pos.x - half_width && self.pos.x < mouse_pos.x + half_width {
            self.pos.y = mouse_pos.y;
        }

        // Update player terrain tracking
        let check_zone = WINDOW_WIDTH / 6.0;
        
        if self.pos.x < check_zone && self.pos.x > check_zone - WINDOW_WIDTH/20.0 {
            player.set_prev_bar_height(self.pos.y);
        }
        if self.pos.x > check_zone && self.pos.x < check_zone + WINDOW_WIDTH/20.0 {
            player.set_bar_height(self.pos.y);
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas, window_height: f32) -> GameResult {
        let bar_rect = Rect::new(
            self.pos.x - self.width/2.0,
            self.pos.y,
            self.width,
            window_height - self.pos.y
        );

        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            bar_rect,
            Color::from_rgb(92, 226, 0), // #5CE200
        )?;
        canvas.draw(&mesh, DrawParam::default());
        Ok(())
    }
}

pub struct Terrain {
    bars: Vec<Bar>,
    bar_width: f32,
}

impl Terrain {
    pub fn new(window_width: f32, initial_height: f32, speed: f32) -> Self {
        let bar_width = window_width / 20.0;
        let mut bars = Vec::new();
        
        // Create initial set of bars
        for i in 0..22 {
            bars.push(Bar::new(
                i as f32 * bar_width,
                initial_height,
                speed,
                bar_width
            ));
        }

        Terrain {
            bars,
            bar_width,
        }
    }

    pub fn update(&mut self, mouse_pos: Vec2, speed_x: f32, ascent: f32, player: &mut Player) {
        for bar in &mut self.bars {
            bar.update(mouse_pos, speed_x, ascent, player);
            
            // Wrap bars around when they go off screen
            if bar.pos.x < -2.0 * self.bar_width {
                bar.pos.x = WINDOW_WIDTH;
                bar.pos.y = mouse_pos.y;
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas, window_height: f32) -> GameResult {
        for bar in &self.bars {
            bar.draw(ctx, canvas, window_height)?;
        }
        Ok(())
    }

    pub fn get_height_at(&self, x: f32) -> Option<f32> {
        for bar in &self.bars {
            if x > bar.pos.x - bar.width/2.0 && x < bar.pos.x + bar.width/2.0 {
                return Some(bar.pos.y);
            }
        }
        None
    }

    pub fn realign(&mut self) {
        if let Some(first_bar) = self.bars.first() {
            let base_x = first_bar.pos.x;
            for (i, bar) in self.bars.iter_mut().enumerate() {
                if bar.pos.x > base_x {
                    bar.pos.x = base_x + (i as f32 * self.bar_width);
                } else {
                    bar.pos.x = base_x - ((22 - i) as f32 * self.bar_width);
                }
            }
        }
    }
}

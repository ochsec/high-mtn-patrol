use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Rect, Mesh, Canvas};
use ggez::glam::Vec2;

pub struct Mountain {
    pub pos: Vec2,
    pub mountain_type: i32,
    segment_width: f32,
    increment: f32,
    max_width: f32,
    color1: Color,
    color2: Color,
}

impl Mountain {
    pub fn new(x: f32, y: f32, mountain_type: i32, window_height: f32) -> Self {
        let segment_width = window_height / 58.0;
        let increment = 3.0 * window_height / 58.0;
        
        // Colors from Processing version
        let (color1, color2) = if mountain_type < 3 {
            (Color::from_rgb(0, 20, 0),    // #001400
             Color::from_rgb(63, 25, 0))   // #3F1900
        } else {
            (Color::from_rgb(63, 25, 0),   // #3F1900
             Color::from_rgb(0, 20, 0))    // #001400
        };

        Mountain {
            pos: Vec2::new(x, y),
            mountain_type,
            segment_width,
            increment,
            max_width: 42.0 * segment_width,
            color1,
            color2,
        }
    }

    pub fn update(&mut self, speed: f32, ascent: f32) {
        if self.mountain_type < 3 {
            self.pos.x -= 0.05 * speed;
            self.pos.y += 0.01 * ascent;
        } else {
            self.pos.x -= 0.25 * speed;
            self.pos.y += 0.05 * ascent;
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        match self.mountain_type {
            0 | 3 => self.draw_mountain_a(ctx, canvas)?,
            1 | 4 => self.draw_mountain_b(ctx, canvas)?,
            2 | 5 => self.draw_mountain_c(ctx, canvas)?,
            _ => {},
        }
        Ok(())
    }

    fn draw_mountain_a(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // First layer
        for i in 0..29 {
            let height = if i <= 14 {
                i as f32 * self.increment
            } else {
                (14.0 * self.increment) - ((i as f32 - 14.0) * self.increment)
            };

            let rect = Rect::new(
                self.pos.x + i as f32 * self.segment_width,
                self.pos.y - height,
                self.segment_width,
                height
            );
            let mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, self.color1)?;
            canvas.draw(&mesh, DrawParam::default());
        }

        // Second layer
        for i in 1..27 {
            let height = if i <= 13 {
                i as f32 * self.increment
            } else {
                (13.0 * self.increment) - ((i as f32 - 13.0) * self.increment)
            };

            let rect = Rect::new(
                self.pos.x + (i as f32 + 1.0) * self.segment_width,
                self.pos.y - height,
                self.segment_width,
                height
            );
            let mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, self.color2)?;
            canvas.draw(&mesh, DrawParam::default());
        }
        Ok(())
    }

    fn draw_mountain_b(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // First layer
        for i in 0..41 {
            let height = if i <= 14 {
                i as f32 * self.increment
            } else if i <= 24 {
                14.0 * self.increment
            } else {
                (14.0 * self.increment) - ((i as f32 - 24.0) * self.increment)
            };

            let rect = Rect::new(
                self.pos.x + i as f32 * self.segment_width,
                self.pos.y - height,
                self.segment_width,
                height
            );
            let mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, self.color1)?;
            canvas.draw(&mesh, DrawParam::default());
        }

        // Second layer
        for i in 1..39 {
            let height = if i <= 13 {
                i as f32 * self.increment
            } else if i <= 23 {
                13.0 * self.increment
            } else {
                (13.0 * self.increment) - ((i as f32 - 23.0) * self.increment)
            };

            let rect = Rect::new(
                self.pos.x + (i as f32 + 1.0) * self.segment_width,
                self.pos.y - height,
                self.segment_width,
                height
            );
            let mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, self.color2)?;
            canvas.draw(&mesh, DrawParam::default());
        }
        Ok(())
    }

    fn draw_mountain_c(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // First peak
        self.draw_mountain_a(ctx, canvas)?;

        // Second peak (offset)
        let offset_x = 14.0 * self.segment_width;
        let offset_y = 3.0 * self.increment;
        let second_peak = Mountain::new(
            self.pos.x + offset_x,
            self.pos.y + offset_y,
            0,
            self.increment * 58.0 / 3.0
        );
        second_peak.draw_mountain_a(ctx, canvas)?;

        Ok(())
    }
}

pub struct Background {
    mountains: Vec<Mountain>,
    window_width: f32,
    window_height: f32,
}

impl Background {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        let mut mountains = Vec::new();
        
        // Create far mountains (types 0-2)
        for i in 0..4 {
            mountains.push(Mountain::new(
                i as f32 * 200.0 + rand::random::<f32>() * 200.0,
                window_height,
                rand::random::<i32>() % 3,
                window_height
            ));
        }
        
        // Create near mountains (types 3-5)
        for i in 4..7 {
            mountains.push(Mountain::new(
                i as f32 * 200.0 + rand::random::<f32>() * 200.0,
                window_height,
                3 + rand::random::<i32>() % 3,
                window_height
            ));
        }

        Background {
            mountains,
            window_width,
            window_height,
        }
    }

    pub fn update(&mut self, speed_x: f32, ascent_speed: f32) {
        for mountain in &mut self.mountains {
            mountain.update(speed_x, ascent_speed);
            
            // Wrap mountains around when they go off screen
            if mountain.pos.x < -mountain.max_width {
                mountain.pos.x = self.window_width + rand::random::<f32>() * 50.0;
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        for mountain in &self.mountains {
            mountain.draw(ctx, canvas)?;
        }
        Ok(())
    }
}

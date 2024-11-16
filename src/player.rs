use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Rect, Mesh};
use glam::Vec2;

pub struct Player {
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
    pub velocity: Vec2,
    pub on_ground: bool,
    pub jump_speed: f32,
    pub move_speed: f32,
    last_y: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            pos: Vec2::new(x, y),
            width: 48.0,
            height: 30.0,
            velocity: Vec2::ZERO,
            on_ground: false,
            jump_speed: -500.0,
            move_speed: 200.0,
            last_y: y,
        }
    }

    pub fn update(&mut self, dt: f32, gravity: f32) {
        self.last_y = self.pos.y;
        
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
        // Calculate rotation based on vertical movement
        let rotation = if self.pos.y > self.last_y {
            -2.0 * std::f32::consts::PI / (self.pos.y / self.last_y)
        } else {
            0.0
        };

        // Colors from the Processing version
        let body_color = Color::from_rgb(127, 183, 190);  // #7FB7BE
        let wheel_color = Color::from_rgb(218, 204, 62);  // #DACC3E
        let window_color = Color::from_rgb(211, 243, 238); // #D3F3EE

        let draw_param = DrawParam::default()
            .dest(self.pos)
            .rotation(rotation)
            .offset(Vec2::new(0.5, 1.0));  // Change y offset to 1.0 to use bottom as anchor

        // Draw body parts
        let body_main = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(10.0, 6.0, 32.0, 14.0),
            body_color,
        )?;
        
        let body_front = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 6.0, 10.0, 10.0),
            body_color,
        )?;

        let cabin_top = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(10.0, 0.0, 20.0, 6.0),
            body_color,
        )?;

        let window = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(18.0, 2.0, 10.0, 4.0),
            window_color,
        )?;

        // Draw wheels (circles)
        let wheel1 = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(14.0, 20.0),
            10.0,
            0.1,
            wheel_color,
        )?;

        let wheel2 = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(38.0, 20.0),
            10.0,
            0.1,
            wheel_color,
        )?;

        // Draw all parts
        canvas.draw(&body_main, draw_param);
        canvas.draw(&body_front, draw_param);
        canvas.draw(&cabin_top, draw_param);
        canvas.draw(&window, draw_param);
        canvas.draw(&wheel1, draw_param);
        canvas.draw(&wheel2, draw_param);

        Ok(())
    }
}

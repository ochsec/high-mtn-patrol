use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Rect, Mesh};
use ggez::glam::Vec2 as GgezVec2;

pub struct Player {
    pub pos: GgezVec2,
    pub width: f32,
    pub height: f32,
    pub velocity: GgezVec2,
    pub on_ground: bool,
    pub jump_speed: f32,
    pub move_speed: f32,
    last_y: f32,
    bar_height: f32,
    prev_bar_height: f32,
    terrain_positions: [f32; 4],  // Track last 4 terrain positions
    speed_y: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            pos: GgezVec2::new(x, y),
            width: 48.0,
            height: 30.0,
            velocity: GgezVec2::ZERO,
            on_ground: false,
            jump_speed: -500.0,
            move_speed: 200.0,
            last_y: y,
            bar_height: y,
            prev_bar_height: y,
            terrain_positions: [y; 4],
            speed_y: 1.0,
        }
    }

    pub fn update(&mut self, dt: f32, gravity: f32, speed_x: &mut f32) {
        self.last_y = self.pos.y;
        
        // Update terrain position history
        for i in (1..4).rev() {
            self.terrain_positions[i] = self.terrain_positions[i-1];
        }
        self.terrain_positions[0] = self.pos.y;

        // Apply gravity and terrain following
        if !self.on_ground {
            self.velocity.y += gravity;
        }
        
        // Update position
        self.pos += self.velocity * dt;

        // Adjust speed based on terrain slope
        if self.prev_bar_height - self.bar_height > self.height {
            *speed_x -= 0.01 * (self.prev_bar_height - self.bar_height);
        } else if self.prev_bar_height <= self.bar_height {
            *speed_x += 0.03 + 0.01 * (self.bar_height - self.prev_bar_height);
        }

        // Cap speed
        if *speed_x < 0.0 {
            *speed_x = 0.0;
        }
        if *speed_x > 20.0 {
            *speed_x = 20.0;
        }
    }

    pub fn set_bar_height(&mut self, height: f32) {
        self.bar_height = height;
    }

    pub fn set_prev_bar_height(&mut self, height: f32) {
        self.prev_bar_height = height;
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
        // Calculate averages for rotation
        let avg0 = (self.terrain_positions[0..3].iter().sum::<f32>()) / 3.0;
        let avg1 = (self.terrain_positions[1..4].iter().sum::<f32>()) / 3.0;
        
        let rotation = if self.pos.y > self.last_y {
            -2.0 * std::f32::consts::PI / (self.pos.y / self.last_y)
        } else {
            -2.0 * std::f32::consts::PI / (avg0 / avg1)
        };

        // Colors from the Processing version
        let body_color = Color::from_rgb(127, 183, 190);  // #7FB7BE
        let wheel_color = Color::from_rgb(218, 204, 62);  // #DACC3E
        let window_color = Color::from_rgb(211, 243, 238); // #D3F3EE

        let draw_param = DrawParam::default()
            .dest(self.pos)
            .rotation(rotation)
            .offset(GgezVec2::new(0.5, 1.0));  // Change y offset to 1.0 to use bottom as anchor

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
            GgezVec2::new(14.0, 20.0),
            10.0,
            0.1,
            wheel_color,
        )?;

        let wheel2 = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            GgezVec2::new(38.0, 20.0),
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

use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Rect, Canvas};
use ggez::glam::Vec2;
use rand::Rng;

pub enum PickupType {
    Coin,
    Gem,
}

pub struct Pickup {
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
    pub pickup_type: PickupType,
    pub collected: bool,
    pub value: i32,
    color: Color,
}

impl Pickup {
    pub fn new(x: f32, y: f32, pickup_type: PickupType) -> Self {
        let (width, height, value, color) = match pickup_type {
            PickupType::Coin => (
                30.0, 
                30.0, 
                25,
                Color::from_rgb(218, 204, 62), // #DACC3E
            ),
            PickupType::Gem => (
                40.0, 
                40.0, 
                50,
                Color::from_rgb(0, 255, 127), // #00FF7F
            ),
        };

        Pickup {
            pos: Vec2::new(x, y),
            width,
            height,
            pickup_type,
            collected: false,
            value,
            color,
        }
    }

    pub fn update(&mut self, speed_x: f32, ascent: f32) {
        self.pos.x -= speed_x;
        self.pos.y += ascent;
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        if !self.collected {
            let pickup_rect = Rect::new(
                self.pos.x,
                self.pos.y,
                self.width,
                self.height
            );

            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                pickup_rect,
                self.color,
            )?;
            canvas.draw(&mesh, DrawParam::default());
        }
        Ok(())
    }

    pub fn reposition(&mut self, window_width: f32, window_height: f32) {
        let mut rng = rand::thread_rng();
        self.pos.x = window_width + rng.gen_range(0.0..200.0);
        self.pos.y = rng.gen_range(0.0..window_height);
        self.collected = false;
    }

    pub fn collides_with_player(&self, player_pos: Vec2, player_width: f32, player_height: f32) -> bool {
        if self.collected {
            return false;
        }

        let player_left = player_pos.x;
        let player_right = player_pos.x + player_width;
        let player_top = player_pos.y - player_height;
        let player_bottom = player_pos.y;
        
        let pickup_left = self.pos.x;
        let pickup_right = self.pos.x + self.width;
        let pickup_top = self.pos.y;
        let pickup_bottom = self.pos.y + self.height;

        player_left < pickup_right && 
        player_right > pickup_left &&
        player_top < pickup_bottom && 
        player_bottom > pickup_top
    }
}

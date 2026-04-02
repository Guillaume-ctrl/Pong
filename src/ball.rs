use crate::Paddle;
use crate::game_manager::GameManager;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub gm: GameManager,
}

impl Ball {
    pub fn new(start_direction : bool,  game_m: GameManager) -> Ball {
        let speed = if start_direction { 4.0 } else { -4.0 };
        Ball {
            x: 400.0, // the game window is 800 long
            y: 300.0, // the game window is 600 high
            r: 15.0,
            x_velocity: speed,
            y_velocity: 0.0,
            gm: game_m,
        }
    }

    pub fn update(&mut self, p1: &Paddle, p2: &Paddle) {
        // move the ball
        self.x += self.x_velocity;
        self.y += self.y_velocity;

        // we need to check if the ball is bouncing agaist a wall ||
        if self.x + self.r >= 800.0 {
            self.gm.touch_right(); // +1 point to right side
            self.x_velocity *= -1.0;
        }
        
        // if it bounce against the walls 
        if self.y + self.r >= 600.0 {
            self.y_velocity *= -1.0;
        }
        // if it bounce against the walls => =
        if self.x - self.r  < 0.0 {
            self.gm.touch_left(); // +1 point to left side
            self.x_velocity *= -1.0;
        }
        // if it bounce against the walls 
        if self.y - self.r <= 0.0 {
            self.y_velocity *= -1.0; 
        }
        
        // Paddle physics

        // left one
        let paddle_center1 = p1.y + (p1.hight / 2.0);
        let d1 = paddle_center1 - self.y;
        // right
        let paddle_center2 = p2.y + (p2.hight / 2.0);
        let d2 = paddle_center2 - self.y;
        
        // left side
        if self.x - self.r <= (p1.x + p1.width) && self.y <= (p1.y + p1.hight) && self.y >= (p1.y) {
            self.y_velocity = -d1 * 0.1;
            self.x_velocity *= -1.0;
        }

        // right side 
        if self.x + self.r >= (p2.x + p2.width) && self.y <= (p2.y + p2.hight) && self.y >= (p2.y) {
            self.y_velocity = -d2 * 0.1;
            self.x_velocity *= -1.0;
        }

    }
}
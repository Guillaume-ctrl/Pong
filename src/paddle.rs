use crate::KeyCode;
use crate::is_key_down;

pub struct Paddle {
    pub y_velocity : f32,
    pub x : f32,
    pub y : f32,
    pub width : f32,
    pub hight : f32,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
}

impl Paddle {
    pub fn new(key_u: KeyCode, key_d: KeyCode, x_pos: f32, y_pos: f32) -> Paddle {
        Paddle {
            y_velocity: 5.0,
            x: x_pos,
            y: y_pos,
            width: 10.0,
            hight: 100.0,
            key_up: key_u,
            key_down: key_d,
        }
    }

    pub fn update(&mut self) {
        
        if is_key_down(self.key_up) {
            self.y -= self.y_velocity;
        }

        else if is_key_down(self.key_down) {
            self.y += self.y_velocity;
        } 

        // limit DOWN
        if self.y + self.hight > 600.0 {
            self.y = 600.0 - self.hight;
        }
        // limit UP
        if self.y < 0.0 {
            self.y = 0.0;
        }
    }
}
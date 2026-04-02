use crate::game_manager::GameManager;
use crate::paddle::Paddle;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub gm: GameManager,
}

impl Ball {
    
    pub fn new(start_direction: bool, game_m: GameManager) -> Result<Self, String> {
        if game_m.finished() {
            return Err("Cannot create ball for a finished game".to_string());
        }
        let speed = if start_direction { 4.0 } else { -4.0 };
        Ok(Ball {
            x: 400.0,
            y: 300.0,
            r: 15.0,
            x_velocity: speed,
            y_velocity: 0.0,
            gm: game_m,
        })
    }

    pub fn update(&mut self, p1: &Paddle, p2: &Paddle) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;

        // vertical Wall Bounce
        if self.y - self.r <= 0.0 || self.y + self.r >= 600.0 {
            self.y_velocity *= -1.0;
        }

        if self.x + self.r >= 800.0 {
            self.gm.touch_left();
            self.reset(false);
        } else if self.x - self.r <= 0.0 {
            self.gm.touch_right();
            self.reset(true);
        }

        // paddle Collision Left
        if self.x - self.r <= p1.x + p1.width
            && self.x - self.r >= p1.x
            && self.y >= p1.y
            && self.y <= p1.y + p1.height
        {
            let paddle_center = p1.y + (p1.height / 2.0);
            self.y_velocity = (self.y - paddle_center) * 0.1;
            self.x_velocity = self.x_velocity.abs();
        }

        // Paddle Collision Right
        if self.x + self.r >= p2.x
            && self.x + self.r <= p2.x + p2.width
            && self.y >= p2.y
            && self.y <= p2.y + p2.height
        {
            let paddle_center = p2.y + (p2.height / 2.0);
            self.y_velocity = (self.y - paddle_center) * 0.1;
            self.x_velocity = -self.x_velocity.abs();
        }
    }

    // reset the ball
    fn reset(&mut self, to_right: bool) {
        self.x = 400.0;
        self.y = 300.0;
        self.y_velocity = 0.0;
        self.x_velocity = if to_right { 4.0 } else { -4.0 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ball_reset() {
        let gm = GameManager::new(5).unwrap();
        let mut ball = Ball::new(true, gm).unwrap();
        ball.x = 900.0;
        ball.reset(false);
        assert_eq!(ball.x, 400.0);
        assert_eq!(ball.x_velocity, -4.0);
    }
}

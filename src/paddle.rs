use macroquad::prelude::*;

/// Represents a player-controlled paddle.
pub struct Paddle {
    pub y_velocity: f32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
}

impl Paddle {
    /// Creates a new Paddle. Returns Err if dimensions are invalid.
    pub fn new(key_u: KeyCode, key_d: KeyCode, x_pos: f32, y_pos: f32) -> Result<Self, String> {
        if x_pos < 0.0 || y_pos < 0.0 {
            return Err("Paddle position cannot be negative".to_string());
        }
        Ok(Paddle {
            y_velocity: 5.0,
            x: x_pos,
            y: y_pos,
            width: 10.0,
            height: 100.0,
            key_up: key_u,
            key_down: key_d,
        })
    }

    /// Updates paddle position based on user input and clamps it to the screen.
    pub fn update(&mut self) {
        if is_key_down(self.key_up) {
            self.y -= self.y_velocity;
        } else if is_key_down(self.key_down) {
            self.y += self.y_velocity;
        }

        self.y = self.y.clamp(0.0, 600.0 - self.height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paddle_creation() {
        let p = Paddle::new(KeyCode::W, KeyCode::S, 10.0, 10.0);
        assert!(p.is_ok());
    }
}

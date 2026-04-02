#[derive(Debug, Clone)]
pub struct GameManager {
    pub current_left_score: u8,
    pub current_right_score: u8,
    pub is_finished: bool,
    pub limit: u8,
}

impl GameManager {
    
    pub fn new(l: u8) -> Result<Self, String> {
        if l == 0 {
            return Err("Score limit must be greater than 0".to_string());
        }
        Ok(GameManager {
            current_left_score: 0,
            current_right_score: 0,
            is_finished: false,
            limit: l,
        })
    }

    pub fn touch_left(&mut self) {
        self.current_left_score += 1;
    }

    pub fn touch_right(&mut self) {
        self.current_right_score += 1;
    }

    pub fn finished(&self) -> bool {
        self.current_right_score >= self.limit || self.current_left_score >= self.limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_manager_scoring() {
        let mut gm = GameManager::new(2).unwrap();
        gm.touch_left();
        assert_eq!(gm.current_left_score, 1);
        assert!(!gm.finished());
        gm.touch_left();
        assert!(gm.finished());
    }

    #[test]
    fn test_invalid_limit() {
        assert!(GameManager::new(0).is_err());
    }
}

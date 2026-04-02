pub struct GameManager {
    pub current_left_score : u8,
    pub current_right_score : u8,
    pub is_finished: bool,
    pub limit: u8,
}

impl GameManager {
    pub fn new(l: u8) -> GameManager {
        GameManager {
            current_left_score: 0,
            current_right_score: 0,
            is_finished: false,
            limit: l,
        }
    }

    pub fn touch_left(&mut self) {
        self.current_left_score += 1;
        // println!("{}", self.current_left_score);
    }
    
    pub fn touch_right(&mut self) {
        self.current_right_score += 1;
        // println!("{}", self.current_left_score);
    }

    pub fn finished(&mut self) -> bool {
        if self.current_right_score > self.limit || self.current_left_score > self.limit {
            return true;
        }
        
        return false;
    }
}
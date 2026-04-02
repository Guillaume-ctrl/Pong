// Encore à faire : Implementer le gamemanager dans le main, creer des méthodes dans le gm
// pour faire gagner ou perdre un joueur, afficher aussi les score.

mod ball;
mod paddle;
mod game_manager;

use macroquad::prelude::*; // engine drawing tools
use ball::Ball;
use paddle::Paddle;
use crate::game_manager::GameManager;


// This sets up our specific 800x600 window
fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

async fn start() -> bool {
    loop {
        // 2. We must clear and draw inside this loop too!
        clear_background(BLACK);
        
        draw_text("HELLO! PRESS RIGHT or LEFT TO PLAY FIRST!", 50.0, 150.0, 40.0, WHITE);

        if is_key_down(KeyCode::Right) {
            return true;
        }
        
        if is_key_down(KeyCode::Left) {
            return false;
        }

        next_frame().await;
    }
}

#[macroquad::main("Pong", window_conf)]
async fn main() {

    //  0                      800
    //  |-----------------------> 
    //  |
    //  |   (To visualize the window)
    //  |
    //  |
    //  v
    // 600

    let start_direction = start().await;
    let mut game_manager = GameManager::new(10);
    let mut my_ball = Ball::new(start_direction, game_manager);
    let mut my_paddle1 = Paddle::new(KeyCode::LeftShift, KeyCode::LeftControl, 15.0, 250.0);
    let mut my_paddle2 = Paddle::new(KeyCode::Up, KeyCode::Down, 775.0, 250.0);

    draw_circle(my_ball.x, my_ball.y, my_ball.r, WHITE);

    loop {
        my_ball.update(&my_paddle1, &my_paddle2);
        my_paddle1.update();
        my_paddle2.update();

        clear_background(BLACK);
        
        let left_score_text = format!("{}", my_ball.gm.current_right_score);
        let right_score_text = format!("{}", my_ball.gm.current_left_score);

        
        draw_text(&left_score_text, 50.0, 150.0, 40.0, WHITE);
        draw_text(&right_score_text, 700.0, 150.0, 40.0, WHITE);

        // top border
        draw_line(0.0, 1.0, 800.0, 1.0, 2.0, WHITE);
        // tottom border
        draw_line(0.0, 599.0, 800.0, 599.0, 2.0, WHITE);
        // center line
        draw_line(400.0, 0.0, 400.0, 600.0, 2.0, WHITE);

        draw_circle(my_ball.x, my_ball.y, 15.0, WHITE);

        draw_rectangle(my_paddle1.x, 
            my_paddle1.y, 
            my_paddle1.width, 
            my_paddle2.hight, 
            WHITE);

        draw_rectangle(my_paddle2.x, 
            my_paddle2.y, 
            my_paddle2.width, 
            my_paddle2.hight, 
            WHITE);

        if my_ball.gm.finished() {
            break;
        }

        next_frame().await;
    }

    loop {
        
        clear_background(BLACK);
        
        draw_text("Finished - Press Enter to exit.", 50.0, 150.0, 40.0, WHITE);

        if is_key_down(KeyCode::Enter) {
            break;
        }

        next_frame().await;
    }
}
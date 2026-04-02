mod ball;
mod game_manager;
mod paddle;

use ball::Ball;
use game_manager::GameManager;
use macroquad::prelude::*;
use paddle::Paddle;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

async fn start_screen() -> bool {
    
    //  0                      800
    //  |-----------------------> 
    //  |
    //  |   (To visualize the window)
    //  |
    //  |
    //  v
    // 600

    loop {
        clear_background(BLACK);
        draw_text(
            "PRESS LEFT OR RIGHT TO START",
            120.0,
            300.0,
            30.0,
            WHITE,
        );
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
async fn main() -> Result<(), String> {
    let start_direction = start_screen().await;
    let game_manager = GameManager::new(10)?;
    let mut my_ball = Ball::new(start_direction, game_manager)?;

    let mut p1 = Paddle::new(KeyCode::LeftShift, KeyCode::LeftControl, 20.0, 250.0)?;
    let mut p2 = Paddle::new(KeyCode::Up, KeyCode::Down, 770.0, 250.0)?;

    loop {
        
        if my_ball.gm.finished() {
            break;
        }

        my_ball.update(&p1, &p2);
        p1.update();
        p2.update();

        clear_background(BLACK);

        draw_line(400.0, 0.0, 400.0, 600.0, 2.0, GRAY);
        draw_text(
            &format!("{}", my_ball.gm.current_left_score),
            200.0,
            100.0,
            50.0,
            WHITE,
        );
        draw_text(
            &format!("{}", my_ball.gm.current_right_score),
            600.0,
            100.0,
            50.0,
            WHITE,
        );

        draw_circle(my_ball.x, my_ball.y, my_ball.r, WHITE);
        draw_rectangle(p1.x, p1.y, p1.width, p1.height, WHITE);
        draw_rectangle(p2.x, p2.y, p2.width, p2.height, WHITE);

        next_frame().await;
    }

    loop {
        clear_background(BLACK);
        draw_text(
            "GAME OVER - PRESS ENTER TO EXIT",
            150.0,
            300.0,
            30.0,
            WHITE,
        );
        if is_key_down(KeyCode::Enter) {
            break;
        }
        next_frame().await;
    }
    Ok(())
}

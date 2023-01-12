use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use rand::Rng;

const GAME_WIDTH: f32 = 384.0;
const GAME_HEIGHT: f32 = 256.0;

const FONT_SIZE: i32 = 20;

const SCORE_HORIZONTAL_DISTANCE: f32 = 30.0;
const SCORE_VERTICAL_DISTANCE: f32 = 30.0;

const NEW_BALL_ANGLE_RANGE: f32 = std::f32::consts::TAU / 4.0;
const DEFAULT_BALL_SPEED: f32 = 160.0;

const PADDLE_DISPLAY_WIDTH: f32 = 3.0;
const DEFAULT_PADDLE_HEIGHT: f32 = 50.0;
const DEFAULT_PADDLE_SPEED: f32 = 100.0;
const DEFAULT_PADDLE_MAX_BOUNCE_ANGLE: f32 = std::f32::consts::TAU / 10.0;

struct Ball {
    position: Vector2,
    velocity: Vector2,
    speed: f32,
    display_radius: f32
}

struct Paddle {
    position: Vector2,
    height: f32,
    speed: f32,
    max_bounce_angle: f32
}

fn new_ball() -> Ball {
    let mut rng = rand::thread_rng();
    let ball_angle = rng.gen::<f32>() * NEW_BALL_ANGLE_RANGE - NEW_BALL_ANGLE_RANGE / 2.0;
    let mut ball_initial_velocity = Vector2::new(f32::cos(ball_angle), f32::sin(ball_angle));
    let side_decider = rng.gen::<f32>();
    if side_decider < 0.5 {
        ball_initial_velocity.x *= -1.0;
    }

    let ball = Ball {
        position: Vector2::new(GAME_WIDTH / 2.0, GAME_HEIGHT / 2.0),
        velocity: ball_initial_velocity,
        speed: DEFAULT_BALL_SPEED,
        display_radius: 3.0,
    };

    return ball;
}

fn new_left_paddle() -> Paddle {
    return Paddle {
        position: Vector2::new(0.0, GAME_HEIGHT / 2.0),
        height: DEFAULT_PADDLE_HEIGHT,
        speed: DEFAULT_PADDLE_SPEED,
        max_bounce_angle: DEFAULT_PADDLE_MAX_BOUNCE_ANGLE
    };
}

fn new_right_paddle() -> Paddle {
    return Paddle {
        position: Vector2::new(GAME_WIDTH, GAME_HEIGHT / 2.0),
        height: DEFAULT_PADDLE_HEIGHT,
        speed: DEFAULT_PADDLE_SPEED,
        max_bounce_angle: DEFAULT_PADDLE_MAX_BOUNCE_ANGLE
    };
}

fn main() {
    let mut left_paddle = new_left_paddle();
    let mut right_paddle = new_right_paddle();
    let mut ball = new_ball();

    let (mut handle, thread) = raylib::init()
        .size(GAME_WIDTH as i32, GAME_HEIGHT as i32)
        .title("Pong")
        .vsync()
        .build();

    let mut left_score: i32 = 0;
    let mut right_score: i32 = 0;

    while !handle.window_should_close() {
        // Update

        // Handle left paddle movement
        let mut left_paddle_movement: f32 = 0.0;
        if handle.is_key_down(KEY_S) {
            left_paddle_movement += 1.0;
        }
        if handle.is_key_down(KEY_W) {
            left_paddle_movement -= 1.0;
        }
        left_paddle_movement *= left_paddle.speed * handle.get_frame_time();
        left_paddle.position.y = f32::max(f32::min(left_paddle.position.y + left_paddle_movement, GAME_HEIGHT - left_paddle.height / 2.0), left_paddle.height / 2.0);

        // Handle right paddle movement
        let mut right_paddle_movement: f32 = 0.0;
        if handle.is_key_down(KEY_DOWN) {
            right_paddle_movement += 1.0;
        }
        if handle.is_key_down(KEY_UP) {
            right_paddle_movement -= 1.0;
        }
        right_paddle_movement *= right_paddle.speed * handle.get_frame_time();
        right_paddle.position.y = f32::max(f32::min(right_paddle.position.y + right_paddle_movement, GAME_HEIGHT - right_paddle.height / 2.0), right_paddle.height / 2.0);

        // Handle ball movement
        ball.velocity = ball.velocity.normalized() * ball.speed;
        ball.position += ball.velocity * handle.get_frame_time();
        
        // Handle ball bouncing and win conditions
        if ball.position.y >= GAME_HEIGHT && ball.velocity.y >= 0.0 {
            ball.velocity.y *= -1.0;
        }
        if ball.position.y <= 0.0 && ball.velocity.y <= 0.0 {
            ball.velocity.y *= -1.0;
        }
        if ball.position.x <= left_paddle.position.x && ball.velocity.x <= 0.0 { // If we've moved off to the left and are moving left
            if left_paddle.position.y - left_paddle.height / 2.0 <= ball.position.y && ball.position.y <= left_paddle.position.y + left_paddle.height / 2.0 { // If we are behind the paddle
                let lerp: f32 = ((ball.position.y - left_paddle.position.y) / (left_paddle.height / 2.0) + 1.0) / 2.0;
                let lerp_a: f32 = -left_paddle.max_bounce_angle;
                let lerp_b: f32 = left_paddle.max_bounce_angle;
                // Treating the lerped angle as a normal to reflect the position with sometimes bounces the ball without flipping its x velocity
                let new_angle: f32 = lerp_a + (lerp_b - lerp_a) * lerp;
                ball.velocity = Vector2::new(f32::cos(new_angle), f32::sin(new_angle));
            } else {
                right_score += 1;
                left_paddle = new_left_paddle();
                right_paddle = new_right_paddle();
                ball = new_ball();
            }
        }
        if ball.position.x >= right_paddle.position.x && ball.velocity.x >= 0.0 { // If we've moved off to the right and are moving right
            if right_paddle.position.y - right_paddle.height / 2.0 <= ball.position.y && ball.position.y <= right_paddle.position.y + right_paddle.height / 2.0 { // If we are behind the paddle
                let lerp: f32 = ((ball.position.y - right_paddle.position.y) / (right_paddle.height / 2.0) + 1.0) / 2.0;
                let lerp_a: f32 = -right_paddle.max_bounce_angle;
                let lerp_b: f32 = right_paddle.max_bounce_angle;
                let new_angle: f32 = lerp_a + (lerp_b - lerp_a) * lerp;
                ball.velocity = Vector2::new(f32::cos(new_angle), f32::sin(new_angle));
                ball.velocity.x *= -1.0;
            } else {
                left_score += 1;
                left_paddle = new_left_paddle();
                right_paddle = new_right_paddle();
                ball = new_ball();
            }
        }

        // Draw

        let mut draw_handle = handle.begin_drawing(&thread);
        draw_handle.clear_background(Color::BLACK);

        draw_handle.draw_rectangle(
            (left_paddle.position.x - PADDLE_DISPLAY_WIDTH / 2.0) as i32, (left_paddle.position.y - DEFAULT_PADDLE_HEIGHT / 2.0) as i32,
            PADDLE_DISPLAY_WIDTH as i32, DEFAULT_PADDLE_HEIGHT as i32,
            Color::WHITE
        );
        draw_handle.draw_rectangle(
            (right_paddle.position.x - PADDLE_DISPLAY_WIDTH / 2.0) as i32, (right_paddle.position.y - DEFAULT_PADDLE_HEIGHT / 2.0) as i32,
            PADDLE_DISPLAY_WIDTH as i32, DEFAULT_PADDLE_HEIGHT as i32,
            Color::WHITE
        );
        draw_handle.draw_circle_v(ball.position, ball.display_radius, Color::WHITE);

        draw_handle.draw_text(&left_score.to_string(), SCORE_HORIZONTAL_DISTANCE as i32, SCORE_VERTICAL_DISTANCE as i32, FONT_SIZE, Color::WHITE);
        let right_score_distance: f32 = GAME_WIDTH - (raylib::core::text::measure_text(&right_score.to_string(), FONT_SIZE) as f32) - SCORE_HORIZONTAL_DISTANCE;
        draw_handle.draw_text(&right_score.to_string(), right_score_distance as i32, SCORE_VERTICAL_DISTANCE as i32, FONT_SIZE, Color::WHITE);
    }
}

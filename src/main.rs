use raylib::prelude::*;
use std::io;

struct BallPrefab{
    posiion: Vector2,
    speed: f32,
    radius: f32
}

fn main() {
    //let mut framecounter = 0;
    let mut x_move: i32 = 1;
    let mut y_move: i32 = -1;


    let screen_width = 800.0;
    let screen_height = 450.0;

    let (mut rl, thread) = raylib::init()
    .size(screen_width as i32, screen_height as i32)
    .title("bouncing ball demo")
    .title("bouncing ball")
    .build();

    rl.set_target_fps(60);

    let mut ball = BallPrefab{
        posiion: Vector2::new( 100.0, 100.0),
        speed: 6.0,
        radius: 30.0,
    };

    let mut vec = (x_move as f32, y_move as f32);

    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);

        Bounce(screen_height, screen_width, x_move as f32, y_move as f32, &mut ball, &mut vec);
        
        draw(&mut d, &mut ball);
    }
}

fn Bounce(
    height: f32,
    width: f32,
    x_move: f32,
    y_move: f32,
    ball: &mut BallPrefab,
    vec: &mut (f32, f32)
){


    if ball.posiion.x - ball.radius < 0.0 || ball.posiion.x + ball.radius > width
    {
        vec.0 = vec.0 * -1.0;
    }
    if ball.posiion.y - ball.radius < 0.0 || ball.posiion.y + ball.radius > height
    {
        vec.1 = vec.1 * -1.0;
    }


    ball.posiion += Vector2::new(vec.0 , vec.1);
}

fn draw(
    d: &mut RaylibDrawHandle,
    ball: &mut BallPrefab
){
    d.clear_background(Color::BLACK);
    d.draw_circle_v(ball.posiion, ball.radius, Color::WHITE);
}

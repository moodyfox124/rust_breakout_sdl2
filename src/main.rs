use std::time::Duration;

use sdl2::video::Window;
use sdl2::{self, event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 400;

const FPS: u32 = 60;
const DELTA_TIME_SEC: f32 = 1.0 / FPS as f32;

const BALL_SPEED: f32 = 400.0;
const BALL_SIZE: u32 = 50;

const BAR_WIDTH: u32 = 200;
const BAR_HIEGHT: u32 = 30;

const BAR_SPEED: f32 = 600.0;

struct Velocity {
    x: f32,
    y: f32,
}

impl Velocity {
    fn new(dx: f32, dy: f32) -> Self {
        Velocity { x: dx, y: dy }
    }
}

struct Position {
    x: f32,
    y: f32,
}

impl Position {
    fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }

    fn calc_position(&mut self, velocity: &mut Velocity) {
        let mut new_x_mov = self.x + velocity.x * BALL_SPEED * DELTA_TIME_SEC;
        if new_x_mov < 0.0 || new_x_mov + BALL_SIZE as f32 > WIDTH as f32 {
            velocity.x = -velocity.x;
            new_x_mov = self.x + velocity.x * BALL_SPEED * DELTA_TIME_SEC;
        }

        let mut new_y_mov = self.y + velocity.y * BALL_SPEED * DELTA_TIME_SEC;
        if new_y_mov < 0.0 || new_y_mov + BALL_SIZE as f32 > HEIGHT as f32 {
            velocity.y = -velocity.y;
            new_y_mov = self.y + velocity.y * BALL_SPEED * DELTA_TIME_SEC;
        }

        self.x = new_x_mov;
        self.y = new_y_mov;
    }
}

fn draw_ball(canvas: &mut Canvas<Window>, ball_pos: &Position) {
    canvas.set_draw_color(Color::RGB(255, 0, 0));

    canvas
        .fill_rect(Rect::new(
            ball_pos.x as i32,
            ball_pos.y as i32,
            BALL_SIZE,
            BALL_SIZE,
        ))
        .unwrap();
}

fn draw_bar(canvas: &mut Canvas<Window>, x: i32, y: i32) {
    canvas.set_draw_color(Color::RGB(50, 200, 0));

    canvas
        .fill_rect(Rect::new(x, y, BAR_WIDTH, BAR_HIEGHT))
        .unwrap();
}

fn calc_ball_mov(ball_pos: &mut Position, velocity: &mut Velocity) {
    let mut new_x_mov = ball_pos.x + velocity.x * BALL_SPEED * DELTA_TIME_SEC;
    if new_x_mov < 0.0 || new_x_mov + BALL_SIZE as f32 > WIDTH as f32 {
        velocity.x = -velocity.x;
        new_x_mov = ball_pos.x + velocity.x * BALL_SPEED * DELTA_TIME_SEC;
    }

    let mut new_y_mov = ball_pos.y + velocity.y * BALL_SPEED * DELTA_TIME_SEC;
    if new_y_mov < 0.0 || new_y_mov + BALL_SIZE as f32 > HEIGHT as f32 {
        velocity.y = -velocity.y;
        new_y_mov = ball_pos.y + velocity.y * BALL_SPEED * DELTA_TIME_SEC;
    }

    ball_pos.x = new_x_mov;
    ball_pos.y = new_y_mov;
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_system = sdl_context.video().unwrap();

    let window = video_system
        .window("breakout", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut ball_position = Position::new(0.0, 0.0);

    let mut bar_x_pos: f32 = (WIDTH - BAR_WIDTH) as f32 / 2.0;
    let bar_y_pos: f32 = (HEIGHT - BAR_HIEGHT) as f32 - 40.0;

    let mut ball_velocity = Velocity::new(1.0, 1.0);

    'mainloop: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    let new_bar = bar_x_pos + BAR_SPEED * DELTA_TIME_SEC;
                    if !(new_bar < -1.0 || new_bar + BAR_WIDTH as f32 > WIDTH as f32) {
                        bar_x_pos = new_bar
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    let new_bar = bar_x_pos - BAR_SPEED * DELTA_TIME_SEC;
                    if !(new_bar < -1.0 || new_bar + BAR_WIDTH as f32 > WIDTH as f32) {
                        bar_x_pos = new_bar
                    }
                }
                // Event::KeyDown {
                //     keycode: Some(Keycode::W),
                //     ..
                // } => {
                //     bar_y_pos -= BAR_SPEED * DELTA_TIME_SEC;
                // }
                // Event::KeyDown {
                //     keycode: Some(Keycode::S),
                //     ..
                // } => {
                //     bar_y_pos += BAR_SPEED * DELTA_TIME_SEC;
                // }
                _ => {}
            }
        }

        draw_bar(&mut canvas, bar_x_pos as i32, bar_y_pos as i32);

        ball_position.calc_position(&mut ball_velocity);

        draw_ball(&mut canvas, &ball_position);

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}

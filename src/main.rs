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
const BAR_HEIGHT: u32 = 30;

const BAR_SPEED: f32 = 600.0;

struct Rectangle {
    velocity: Velocity,
    position: Position,
    size: Size,
    color: Color,
}

impl Rectangle {
    fn new(velocity: Velocity, position: Position, size: Size, color: Color) -> Self {
        Self {
            velocity,
            position,
            size,
            color,
        }
    }
}

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

struct Size {
    width: u32,
    height: u32,
}

impl Size {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn draw(canvas: &mut Canvas<Window>, ball_pos: &Position, size: &Size, color: &Color) {
    canvas.set_draw_color(color.clone());

    canvas
        .fill_rect(Rect::new(
            ball_pos.x as i32,
            ball_pos.y as i32,
            size.width,
            size.height,
        ))
        .unwrap();
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

    let mut ball = Rectangle::new(
        Velocity::new(1.0, 1.0),
        Position::new(0.0, 0.0),
        Size::new(BALL_SIZE, BALL_SIZE),
        Color::RGB(255, 0, 0),
    );

    let mut bar_position = Position::new(
        (WIDTH - BAR_WIDTH) as f32 / 2.0,
        (HEIGHT - BAR_HEIGHT) as f32 - 40.0,
    );

    let bar_size = Size::new(BAR_WIDTH, BAR_HEIGHT);
    let bar_color = Color::RGB(50, 200, 0);

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
                    let new_bar = bar_position.x + BAR_SPEED * DELTA_TIME_SEC;
                    if !(new_bar < -1.0 || new_bar + BAR_WIDTH as f32 > WIDTH as f32) {
                        bar_position.x = new_bar
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    let new_bar = bar_position.x - BAR_SPEED * DELTA_TIME_SEC;
                    if !(new_bar < -1.0 || new_bar + BAR_WIDTH as f32 > WIDTH as f32) {
                        bar_position.x = new_bar
                    }
                }
                _ => {}
            }
        }

        draw(&mut canvas, &bar_position, &bar_size, &bar_color);

        ball.position.calc_position(&mut ball.velocity);

        draw(&mut canvas, &ball.position, &ball.size, &ball.color);

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}

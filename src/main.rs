extern crate sdl2;

use sdl2::VideoSubsystem;
use sdl2::event::Event;
use sdl2::hint::set_video_minimize_on_focus_loss_with_priority;
use sdl2::keyboard::{KeyboardState, Keycode};
use sdl2::libc::time_t;
use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use std::os::linux::raw::stat;
use std::time::{self, Duration};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

enum GameState {
    Start,
    Play,
}
#[derive(Debug)]
struct Pos {
    x: f32,
    y: f32,
}
#[derive(Debug)]
struct ColorM {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
#[derive(Debug)]
struct Ball {
    pos: Pos,
    radius: f32,
    xv: f32,
    yv: f32,
    color: ColorM,
}

impl Ball {
    fn new(x: f32, y: f32, radius: f32, xv: f32, yv: f32, color: ColorM) -> Self {
        Self {
            pos: Pos { x: x, y: y },
            radius: radius,
            xv: xv,
            yv: yv,
            color: color,
        }
    }
    fn draw(&self, pixels: &mut Vec<u8>) {
        //need to implement by fully understanding it
        for y in -(self.radius) as i32..=(self.radius) as i32 {
            for x in -(self.radius) as i32..=(self.radius) as i32 {
                if x * x + y * y < self.radius as i32 * self.radius as i32 {
                    //let make sure our band we want the x and y to be less that width and height
                    let dx = self.pos.x as i32 + x;
                    let dy = self.pos.y as i32 + y;
                    if dx >= 0 && dx < WIDTH as i32 && dy >= 0 && dy < HEIGHT as i32 {
                        let starting_x = (dy * 800 as i32 + dx) * 4; //this is where the starting is
                        //located when flattned
                        pixels[starting_x as usize] = self.color.r;
                        pixels[(starting_x + 1) as usize] = self.color.g;
                        pixels[(starting_x + 2) as usize] = self.color.b;
                        pixels[(starting_x + 3) as usize] = self.color.a;
                    }
                }
            }
        }
    }

    fn reset(&mut self) {
        self.pos.x = WIDTH as f32 / 2.0;
        self.pos.y = HEIGHT as f32 / 2.0;
        self.xv = -200.0; // Reset to default velocity
        self.yv = -200.0;
    }
    fn update(
        &mut self,
        left_paddle: &Paddle,
        right_paddle: &Paddle,
        elapsed_time: time::Duration,
    ) {
        self.pos.x += self.xv * elapsed_time.as_secs_f32();
        self.pos.y += self.yv * elapsed_time.as_secs_f32();

        if self.pos.y - self.radius <= 0.0 || self.pos.y + self.radius > HEIGHT as f32 {
            self.yv = -self.yv;
        }

        if self.pos.x < 0.0 || self.pos.x > WIDTH as f32 {
            self.reset(); //recenter its 
        }
        //lets handle the collision
        //for left paddle first
        //ball.x - radius <= leftpaddle.x + w => collision on x axis || ball.y - self.radius <=
        //left_paddle.y - height  || ball.y + self.radius <= lef_paddle.y _ height => left paddle collision
        if self.pos.x - self.radius <= left_paddle.pos.x + left_paddle.width as f32 / 2.0
            && self.pos.x + self.radius >= left_paddle.pos.x - left_paddle.width as f32 / 2.0
        {
            if self.pos.y - self.radius >= left_paddle.pos.y - left_paddle.height as f32 / 2.0
                && self.pos.y + self.radius <= left_paddle.pos.y + left_paddle.height as f32 / 2.0
            {
                self.xv = -self.xv;
                self.pos.x = left_paddle.pos.x + left_paddle.width as f32 / 2.0 + self.radius;
            }
        }

        if self.pos.x + self.radius >= right_paddle.pos.x + right_paddle.width as f32 / 2.0
            && self.pos.x - self.radius <= right_paddle.pos.x + right_paddle.width as f32 / 2.0
        {
            if self.pos.y - self.radius >= right_paddle.pos.y - right_paddle.height as f32 / 2.0
                && self.pos.y + self.radius <= right_paddle.pos.y + right_paddle.height as f32 / 2.0
            {
                self.xv = -self.xv;
                self.pos.x = right_paddle.pos.x - right_paddle.width as f32 / 2.0 - self.radius;
            }
        }
    }
}
#[derive(Debug)]
struct Paddle {
    pos: Pos,
    width: u32,
    height: u32,
    speed: f32,
    color: ColorM,
    score: i32,
}

impl Paddle {
    fn new(x: f32, y: f32, width: u32, height: u32, color: ColorM) -> Self {
        Self {
            pos: Pos { x: x, y: y },
            width: width,
            height: height,
            speed: 700.0,
            color: color,
            score: 0,
        }
    }

    fn draw(&self, pixels: &mut Vec<u8>) {
        let left_paddle = self.pos.x as f32 - (self.width as f32 / 2.0);
        let top_paddle = self.pos.y as f32 - (self.height as f32 / 2.0);
        let right_paddle = left_paddle + self.width as f32;
        let bottom_paddle = top_paddle + self.height as f32;
        println!(
            "== LEFT = {} TOP = {} RIGHT = {} BOTTOM = {}",
            left_paddle, top_paddle, right_paddle, bottom_paddle
        );
        //lets draw on the pixel
        for y in top_paddle as i32..bottom_paddle as i32 {
            for x in left_paddle as i32..right_paddle as i32 {
                if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                    //starting index of the x;
                    let starting_x = (y * 800 as i32 + x) * 4; //this is where the starting is
                    //located when flattned
                    pixels[starting_x as usize] = self.color.r;
                    pixels[(starting_x + 1) as usize] = self.color.g;
                    pixels[(starting_x + 2) as usize] = self.color.b;
                    pixels[(starting_x + 3) as usize] = self.color.a;
                }
            }
        }
    }
    fn update(&mut self, keyboard_state: KeyboardState, elapsed_time: time::Duration) {
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Up) {
            self.pos.y -= self.speed * elapsed_time.as_secs_f32();

            if self.pos.y - self.height as f32 / 2.0 < 0.0 {
                self.pos.y = self.height as f32 / 2.0;
            }
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Down) {
            self.pos.y += self.speed * elapsed_time.as_secs_f32();
            if self.pos.y + self.height as f32 / 2.0 > HEIGHT as f32 {
                self.pos.y = HEIGHT as f32 - self.height as f32 / 2.0;
            }
        }
    }

    fn ai_update(&mut self, ball: &Ball) {
        self.pos.y = ball.pos.y;
    }
}

pub fn main() {
    let mut pixels: Vec<u8> = vec![0; 800 * 600 * 4];
    println!("PIXELS: {:?}", pixels);
    let sdl_context = match sdl2::init() {
        Ok(initialized) => initialized,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    /*
     *   a) Window Management => main purpose to faciliate creation of window and management of
     *   windows
     *
     *   b) Display Information => allows you to query information about the connected dispaly
     *
     *   c) Hardware Acceleration integration => crucial for integrating iwth moder engines
     *
     *   d) Clipbaord and text input => manages clipboard functionality and provices access to text
     *   input utilities for handling user text entry
     *
     *   e) context for other video related types => objects like window and windowbuilder
     */
    let video_subsystem: VideoSubsystem = match sdl_context.video() {
        Ok(vs) => vs,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    //window part
    let window = video_subsystem
        .window("PING PONG", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    //
    //drawing canvas
    //
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ABGR8888, 800, 600)
        .unwrap();
    //i am given a cavas on the window to draw
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut new_paddle_1 = Paddle::new(
        100.0,
        100.0,
        20,
        100,
        ColorM {
            r: 0,
            g: 255,
            b: 255,
            a: 255,
        },
    );
    let mut new_paddle_2 = Paddle::new(
        WIDTH as f32 - 100.0,
        100.0,
        20,
        100,
        ColorM {
            r: 0,
            g: 255,
            b: 255,
            a: 255,
        },
    );
    let new_ball_center_x = (new_paddle_2.pos.x + new_paddle_1.pos.x) / 2.0;
    let new_ball_center_y = new_paddle_1.pos.y;
    let mut new_ball_1 = Ball::new(
        new_ball_center_x,
        new_ball_center_y,
        20.0,
        200.0,
        200.0,
        ColorM {
            r: 0,
            g: 255,
            b: 255,
            a: 255,
        },
    );
    let mut event_dump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut state_game = GameState::Start;
    let mut last_frame_time = time::Instant::now();
    'running: loop {
        let elapsed = last_frame_time.elapsed();
        last_frame_time = time::Instant::now();
        i = (i + 1) % 255;

        //canvas.set_draw_color(Color::RGB(i, 64, 255 - 1));
        //canvas.clear();

        for event in event_dump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let keboard_state = event_dump.keyboard_state();
        if keboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Down) {
            println!("Moving DOWN");
        } else if keboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Up) {
            println!("Moving UP");
        }
        for pixel in pixels.iter_mut() {
            *pixel = 0;
        }
        match state_game {
            GameState::Play => {
                new_paddle_1.update(keboard_state, elapsed);
                new_paddle_2.ai_update(&mut new_ball_1);
                new_ball_1.update(&new_paddle_1, &new_paddle_2, elapsed);
            }
            GameState::Start => {
                state_game = GameState::Play;
            }
        }
        new_paddle_1.draw(&mut pixels);
        new_paddle_2.draw(&mut pixels);
        new_ball_1.draw(&mut pixels);
        texture.update(None, &pixels, 800 * 4).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        ::std::thread::sleep(time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Cleaned up successfully!");
}

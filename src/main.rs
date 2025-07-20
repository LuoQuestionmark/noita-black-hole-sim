extern crate sdl2;

use std::env;
use std::time::Duration;

use itertools::iproduct;
use libm::{atan2f, cosf, fabsf, sinf};
use rand::Rng;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Copy, Clone)]
struct Vec2D {
    x: f32,
    y: f32,
}

const G: f32 = 1000.0;
const DRAG: f32 = 0.001;
const C: f32 = 1000.0;

const X_MIN: f32 = 0.0;
const X_MAX: f32 = 100.0;
const Y_MIN: f32 = 0.0;
const Y_MAX: f32 = 100.0;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

fn init_position(pos: &mut Vec<Vec2D>, count: u8) {
    for _ in 0..count {
        let a = rand::rng().random_range(X_MIN..X_MAX);
        let b = rand::rng().random_range(Y_MIN..Y_MAX);
        let p = Vec2D { x: a, y: b };
        pos.push(p);
    }
}

fn init_speed(spd: &mut Vec<Vec2D>, count: u8) {
    for _ in 0..count {
        let v = Vec2D { x: 0.0, y: 0.0 };
        spd.push(v);
    }
}

fn init_acceleration(acc: &mut Vec<Vec2D>, count: u8) {
    for _ in 0..count {
        let a = Vec2D { x: 0.0, y: 0.0 };
        acc.push(a);
    }
}

fn boundary_check(pos: &mut Vec<Vec2D>, spd: &mut Vec<Vec2D>) {
    for i in 0..pos.len() {
        if pos[i].x.is_nan() {
            let a = rand::rng().random_range(X_MIN..X_MAX);
            let b = rand::rng().random_range(Y_MIN..Y_MAX);
            pos[i] = Vec2D { x: a, y: b };
            spd[i] = Vec2D { x: 0.0, y: 0.0 };
        }

        if pos[i].y.is_nan() {
            let a = rand::rng().random_range(X_MIN..X_MAX);
            let b = rand::rng().random_range(Y_MIN..Y_MAX);
            pos[i] = Vec2D { x: a, y: b };
            spd[i] = Vec2D { x: 0.0, y: 0.0 };
        }

        if spd[i].x.is_nan() || fabsf(spd[i].x) > C {
            let a = rand::rng().random_range(X_MIN..X_MAX);
            let b = rand::rng().random_range(Y_MIN..Y_MAX);
            pos[i] = Vec2D { x: a, y: b };
            spd[i] = Vec2D { x: 0.0, y: 0.0 };
        }

        if spd[i].y.is_nan() || fabsf(spd[i].y) > C {
            let a = rand::rng().random_range(X_MIN..X_MAX);
            let b = rand::rng().random_range(Y_MIN..Y_MAX);
            pos[i] = Vec2D { x: a, y: b };
            spd[i] = Vec2D { x: 0.0, y: 0.0 };
        }

        if pos[i].x < X_MIN {
            spd[i].x = fabsf(spd[i].x);
        }
        if pos[i].x > X_MAX {
            spd[i].x = -fabsf(spd[i].x);
        }
        if pos[i].y < Y_MIN {
            spd[i].y = fabsf(spd[i].y);
        }
        if pos[i].y > X_MAX {
            spd[i].y = -fabsf(spd[i].y);
        }
    }
}

fn update(pos: &mut Vec<Vec2D>, spd: &mut Vec<Vec2D>, acc: &mut Vec<Vec2D>, delta: f32) {
    let size = pos.len();
    for i in 0..size {
        acc[i] = Vec2D { x: 0.0, y: 0.0 };
    }

    // update acceleration
    for (i, j) in iproduct!(0..size, 0..size) {
        if i == j {
            continue;
        }

        let mut dist_square: f32 =
            f32::powf(pos[i].x - pos[j].x, 2.0) + f32::powf(pos[i].y - pos[j].y, 2.0);

        if dist_square < 1.0 {
            dist_square = 1.0;
        }

        let acc_norm: f32 = G / dist_square;

        let angle = atan2f(pos[i].y - pos[j].y, pos[i].x - pos[j].x);

        acc[i] = Vec2D {
            x: acc[i].x - acc_norm * cosf(angle),
            y: acc[i].y - acc_norm * sinf(angle),
        };
    }

    for i in 0..size {
        // println!(
        //     "{:.2} {:.2} {:.2} {:.2}",
        //     acc[i].x,
        //     DRAG * spd[i].x,
        //     acc[i].y,
        //     DRAG * spd[i].y
        // );
        acc[i] = Vec2D {
            x: acc[i].x - DRAG * spd[i].x,
            y: acc[i].y - DRAG * spd[i].y,
            // x: acc[i].x - DRAG * spd[i].x * spd[i].x,
            // y: acc[i].y - DRAG * spd[i].y * spd[i].y,
        };
    }

    // update speed
    for i in 0..size {
        spd[i] = Vec2D {
            x: spd[i].x + acc[i].x * delta,
            y: spd[i].y + acc[i].y * delta,
        };
    }

    // update pos
    for i in 0..size {
        pos[i] = Vec2D {
            x: pos[i].x + spd[i].x * delta,
            y: pos[i].y + spd[i].y * delta,
        };
    }

    boundary_check(pos, spd);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let count = args[1].trim().parse::<u8>().unwrap();
    let count = 10;

    let mut pos = Vec::new();
    let mut spd = Vec::new();
    let mut acc = Vec::new();

    init_position(&mut pos, count);
    init_speed(&mut spd, count);
    init_acceleration(&mut acc, count);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("black holes", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'execution: loop {
        update(&mut pos, &mut spd, &mut acc, 0.01);

        canvas.set_draw_color(Color::RGB(220, 220, 220));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'execution,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for p in &pos {
            let x: i32 = ((p.x - X_MIN) / (X_MAX - X_MIN) * (WINDOW_WIDTH as f32)) as i32;
            let y: i32 = ((p.y - Y_MIN) / (Y_MAX - Y_MIN) * (WINDOW_HEIGHT as f32)) as i32;
            let _ = canvas.fill_rect(Rect::new(x, y, 20, 20));
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
    }
}

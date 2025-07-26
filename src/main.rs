extern crate sdl2;

// use std::env;
use std::time::Duration;

use itertools::iproduct;
use libm::{atan2f, cosf, fabsf, sinf, sqrtf};
use rand::Rng;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::surface::Surface;
use sdl2::video::Window;

#[derive(Copy, Clone)]
struct Vec3D {
    x: f32,
    y: f32,
    z: f32,
}

const G: f32 = 2000.0;
const DRAG: f32 = 0.001;
const C: f32 = 10000.0;

const X_MIN: f32 = 0.0;
const X_MAX: f32 = 100.0;
const Y_MIN: f32 = 0.0;
const Y_MAX: f32 = 100.0;
const Z_MIN: f32 = 0.0;
const Z_MAX: f32 = 100.0;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

fn init_position(pos: &mut Vec<Vec3D>, count: u8) {
    for _ in 0..count {
        let a = rand::rng().random_range(X_MIN..X_MAX);
        let b = rand::rng().random_range(Y_MIN..Y_MAX);
        let c = rand::rng().random_range(Z_MIN..Z_MAX);
        let p = Vec3D { x: a, y: b, z: c };
        pos.push(p);
    }
}

fn init_speed(spd: &mut Vec<Vec3D>, count: u8) {
    for _ in 0..count {
        let v = Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        spd.push(v);
    }
}

fn init_acceleration(acc: &mut Vec<Vec3D>, count: u8) {
    for _ in 0..count {
        let a = Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        acc.push(a);
    }
}

fn boundary_check(pos: &mut Vec<Vec3D>, spd: &mut Vec<Vec3D>) {
    for i in 0..pos.len() {
        let mut reset: bool = false;
        if pos[i].x.is_nan() {
            reset = true;
        } else if pos[i].y.is_nan() {
            reset = true;
        } else if pos[i].y.is_nan() {
            reset = true;
        } else if spd[i].x.is_nan() || fabsf(spd[i].x) > C {
            reset = true;
        } else if spd[i].y.is_nan() || fabsf(spd[i].y) > C {
            reset = true;
        }

        if reset {
            let a = rand::rng().random_range(X_MIN..X_MAX);
            let b = rand::rng().random_range(Y_MIN..Y_MAX);
            let c = rand::rng().random_range(Z_MIN..Z_MAX);
            pos[i] = Vec3D { x: a, y: b, z: c };
            spd[i] = Vec3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
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
        if pos[i].z < Z_MIN {
            spd[i].z = fabsf(spd[i].z);
        }
        if pos[i].z > Z_MAX {
            spd[i].z = -fabsf(spd[i].z);
        }
    }
}

fn update(pos: &mut Vec<Vec3D>, spd: &mut Vec<Vec3D>, acc: &mut Vec<Vec3D>, delta: f32) {
    let size = pos.len();
    for i in 0..size {
        acc[i] = Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    // update acceleration
    for (i, j) in iproduct!(0..size, 0..size) {
        if i == j {
            continue;
        }

        let mut dist_square: f32 = f32::powf(pos[i].x - pos[j].x, 2.0)
            + f32::powf(pos[i].y - pos[j].y, 2.0)
            + f32::powf(pos[i].z - pos[j].z, 2.0);

        if dist_square < 1.0 {
            dist_square = 1.0;
        }

        let acc_norm: f32 = G / dist_square;

        let x = pos[j].x - pos[i].x;
        let y = pos[j].y - pos[i].y;
        let z = pos[j].z - pos[i].z;

        let theta = atan2f(x, sqrtf(y * y + z * z));
        let phi = atan2f(z, y);

        acc[i] = Vec3D {
            x: acc[i].x + acc_norm * sinf(theta),
            y: acc[i].y + acc_norm * cosf(theta) * cosf(phi),
            z: acc[i].y + acc_norm * cosf(theta) * sinf(phi),
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
        acc[i] = Vec3D {
            x: acc[i].x - DRAG * spd[i].x,
            y: acc[i].y - DRAG * spd[i].y,
            z: acc[i].z - DRAG * spd[i].z,
            // x: acc[i].x - DRAG * spd[i].x * spd[i].x,
            // y: acc[i].y - DRAG * spd[i].y * spd[i].y,
        };
    }

    // update speed
    for i in 0..size {
        spd[i] = Vec3D {
            x: spd[i].x + acc[i].x * delta,
            y: spd[i].y + acc[i].y * delta,
            z: spd[i].z + acc[i].z * delta,
        };
    }

    // update pos
    for i in 0..size {
        pos[i] = Vec3D {
            x: pos[i].x + spd[i].x * delta,
            y: pos[i].y + spd[i].y * delta,
            z: pos[i].z + spd[i].z * delta,
        };
    }

    boundary_check(pos, spd);
}

fn canvas_draw_cube(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(50, 50, 50));

    canvas
        .draw_line(
            Point::new((WINDOW_WIDTH / 2) as i32, (WINDOW_HEIGHT / 2) as i32),
            Point::new(
                (WINDOW_WIDTH / 2) as i32 + 344,
                (WINDOW_WIDTH / 2) as i32 - 200,
            ),
        )
        .unwrap();
    canvas
        .draw_line(
            Point::new((WINDOW_WIDTH / 2) as i32, (WINDOW_HEIGHT / 2) as i32),
            Point::new(
                (WINDOW_WIDTH / 2) as i32 - 344,
                (WINDOW_WIDTH / 2) as i32 - 200,
            ),
        )
        .unwrap();
    canvas
        .draw_line(
            Point::new((WINDOW_WIDTH / 2) as i32, (WINDOW_HEIGHT / 2) as i32),
            Point::new((WINDOW_WIDTH / 2) as i32, (WINDOW_WIDTH / 2) as i32 + 400),
        )
        .unwrap();
    canvas
        .draw_line(
            Point::new(
                (WINDOW_WIDTH / 2) as i32 - 344,
                (WINDOW_HEIGHT / 2) as i32 - 200,
            ),
            Point::new(
                (WINDOW_WIDTH / 2) as i32 - 344,
                (WINDOW_WIDTH / 2) as i32 + 200,
            ),
        )
        .unwrap();
    canvas
        .draw_line(
            Point::new(
                (WINDOW_WIDTH / 2) as i32 + 344,
                (WINDOW_HEIGHT / 2) as i32 - 200,
            ),
            Point::new(
                (WINDOW_WIDTH / 2) as i32 + 344,
                (WINDOW_WIDTH / 2) as i32 + 200,
            ),
        )
        .unwrap();
    canvas
        .draw_line(
            Point::new(
                (WINDOW_WIDTH / 2) as i32 + 344,
                (WINDOW_HEIGHT / 2) as i32 - 200,
            ),
            Point::new((WINDOW_WIDTH / 2) as i32, (WINDOW_WIDTH / 2) as i32 - 400),
        )
        .unwrap();
    canvas
        .draw_line(
            Point::new(
                (WINDOW_WIDTH / 2) as i32 - 344,
                (WINDOW_HEIGHT / 2) as i32 - 200,
            ),
            Point::new((WINDOW_WIDTH / 2) as i32, (WINDOW_WIDTH / 2) as i32 - 400),
        )
        .unwrap();
    canvas
        .draw_line(
            Point::new(
                (WINDOW_WIDTH / 2) as i32 + 344,
                (WINDOW_HEIGHT / 2) as i32 + 200,
            ),
            Point::new((WINDOW_WIDTH / 2) as i32, (WINDOW_WIDTH / 2) as i32 + 400),
        )
        .unwrap();
    canvas
        .draw_line(
            Point::new(
                (WINDOW_WIDTH / 2) as i32 - 344,
                (WINDOW_HEIGHT / 2) as i32 + 200,
            ),
            Point::new((WINDOW_WIDTH / 2) as i32, (WINDOW_WIDTH / 2) as i32 + 400),
        )
        .unwrap();
}

fn canvas_render_black_hole(canvas: &mut Canvas<Window>, position: Vec3D, texture: &Texture) {
    let u: i32 = (0.866 * (-position.x + position.y) * 0.005 * (WINDOW_WIDTH as f32)) as i32
        + (WINDOW_WIDTH as i32) / 2;
    let v: i32 = ((0.5 * (position.x + position.y) - position.z) * 0.005 * (WINDOW_HEIGHT as f32))
        as i32
        + (WINDOW_HEIGHT as i32) / 2;
    let rect = Rect::new(u - 10, v - 10, 20, 20);
    canvas.copy(texture, None, rect).unwrap();
}

fn main() {
    // let args: Vec<String> = env::args().collect();
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
    let texture_creator = canvas.texture_creator();

    sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();
    let texture = texture_creator.load_texture("resources/bh.png").unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'execution: loop {
        // update physics
        update(&mut pos, &mut spd, &mut acc, 0.01);

        // background color
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // draw the cube that gives a visual hint
        canvas_draw_cube(&mut canvas);

        // copied from example, click escape to stop execution
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

        // draw "black holes"
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for p in &pos {
            canvas_render_black_hole(&mut canvas, *p, &texture);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
    }
}

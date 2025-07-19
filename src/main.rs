use std::env;
use libm::{sinf, cosf, atan2f, fabsf};
use itertools::iproduct;
use rand::Rng;

#[derive(Copy)]
#[derive(Clone)]
struct Vec2D {
    x: f32,
    y: f32,
}

const G:f32 = 1.0;
const X_MIN: f32 = 0.0;
const X_MAX: f32 = 100.0;
const Y_MIN: f32 = 0.0;
const Y_MAX: f32 = 100.0;

fn init_position(pos: &mut Vec<Vec2D>, count: u8) {
    for i in 0..count {
        let a = rand::rng().random_range(X_MIN..X_MAX);
        let b = rand::rng().random_range(Y_MIN..Y_MAX);
        let p = Vec2D {x: a, y: b};
        pos.push(p);
    }
}

fn init_speed(spd: &mut Vec<Vec2D>, count: u8) {
    for i in 0..count {
        let v = Vec2D {x: 0.0, y: 0.0};
        spd.push(v);
    }
}

fn init_acceleration(acc: &mut Vec<Vec2D>, count: u8) {
    for i in 0..count {
        let a = Vec2D {x: 0.0, y: 0.0};
        acc.push(a);
    }
}

fn boundary_check(pos: &mut Vec<Vec2D>, spd: &mut Vec<Vec2D>) {
    for i in 0..pos.len() {
        if pos[i].x < X_MIN {
            spd[i].x = fabsf(spd[i].x);
        }
        if pos[i].x > X_MAX {
            spd[i].x = - fabsf(spd[i].x);
        }
        if pos[i].y < Y_MIN {
            spd[i].y = fabsf(spd[i].y);
        }
        if pos[i].y > X_MAX {
            spd[i].y = - fabsf(spd[i].y);
        }
    }
}


fn update(pos: &mut Vec<Vec2D>, spd: &mut Vec<Vec2D>, acc: &mut Vec<Vec2D>, delta: f32) {
    let size = pos.len();
    for i in 0..size {
        acc[i] = Vec2D {x: 0.0, y: 0.0};
    }

    // update acceleration
    for (i, j) in iproduct!(0..size, 0..size) {
        if i == j {continue;}

        let dist_square: f32 = f32::powf(pos[i].x - pos[j].x, 2.0) + f32::powf(pos[i].y - pos[j].y, 2.0);
        let acc_norm: f32 = G / dist_square;

        let angle = atan2f(pos[i].y - pos[j].y, pos[i].x - pos[j].x);
        acc[i] = Vec2D {x: acc[i].x + acc_norm * cosf(angle), y: acc[i].y + acc_norm * sinf(angle)};
    }

    // update speed
    for i in 0..size {
        spd[i] = Vec2D {x: spd[i].x + acc[i].x * delta, y: spd[i].y + acc[i].y * delta};
    }

    // update pos
    for i in 0..size {
        pos[i] = Vec2D {x: pos[i].x + spd[i].x * delta, y: pos[i].y + spd[i].y * delta};
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

    loop {
        update(&mut pos, &mut spd, &mut acc, 0.01);
        // println!("x {} y {}", pos[0].x, pos[0].y);
        println!("x {} y {}", spd[0].x, spd[0].y);
        // println!("x {} y {}", acc[0].x, acc[0].y);
    }
}

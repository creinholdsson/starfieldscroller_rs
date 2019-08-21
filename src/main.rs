extern crate minifb;
use minifb::{Window, Key, Scale, WindowOptions};

extern crate stopwatch;
use stopwatch::{Stopwatch};

use rand::prelude::*;

const WIDTH: usize = 512;
const HEIGHT: usize = 256;

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,
    color: u32,
    speed:f32,
}

fn update(particles: &mut Vec<Particle>) {
    for p in particles.iter_mut() {
        p.x += p.speed;
        if p.x as usize >= WIDTH {
            p.x = 0.0;
        }
    }
}

fn render_clear_buffer(render_buffer: &mut Vec<u32>, color: u32) {
    for i in render_buffer.iter_mut() {
        *i = color;
    }
}

fn render_draw_particles(render_buffer: &mut Vec<u32>, particles: &Vec<Particle>) {
    let w = WIDTH as f32;
    for p in particles.iter() {
        let i = (p.x + p.y * w) as usize;
        render_buffer[i] = p.color;
    }
}

fn render_draw(render_buffer: &mut Vec<u32>, particles: &Vec<Particle>) {
    render_clear_buffer(render_buffer, 0);
    render_draw_particles(render_buffer, particles);
}

fn main() {
    let mut window = match Window::new("Starfield - Press ESC to exit", WIDTH, HEIGHT,
                                       WindowOptions {
                                           resize: false,
                                           scale: Scale::X2,
                                           ..WindowOptions::default()
                                       }) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    let mut particles: Vec<Particle> = vec![Particle{ x:0.0, y: 0.0, color: 0xffffff, speed: 0.0}; HEIGHT];
    let mut rng = thread_rng();
    let mut y = 0.0;

    for p in particles.iter_mut() {
        p.x = rng.gen_range(0.0, 1.0 * (WIDTH - 1) as f32);
        p.y = y;
        let speed = rng.gen_range(0.1, 1.0);
        let channel: u32 = (speed * 255.0) as u32;

        p.color = ((channel << 16) | (channel << 8) | channel) as u32;
        p.speed = speed;
        y += 1.0;
    }

    let mut buffer: Vec<u32> = vec![0; WIDTH*HEIGHT];
    let mut sw = Stopwatch::new();

    sw.start();
    let mut accumulator:i64 = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        sw.stop();

        let d = sw.elapsed_ms();
        accumulator += d;

        sw.restart();

        while accumulator >= 16 {
            accumulator -= 16;
            update(&mut particles);
        }
        render_draw(&mut buffer, &particles);

        window.update_with_buffer(&buffer).unwrap();
    }
}

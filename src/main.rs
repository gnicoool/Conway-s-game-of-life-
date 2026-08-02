use minifb::{Key, Window, WindowOptions};

use std::time::Duration;

mod framebuffer;
mod conway;
mod organismos;

use crate::framebuffer::Framebuffer;
use crate::conway::actualizar_celulas;
use crate::organismos::{blinker, glider, toad, loaf, flor};

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway´s game of life",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(0x222538);
    framebuffer.clear();
    framebuffer.set_current_color(0xf7a5dc);

    let frame_delay = Duration::from_millis(100);
    let center_x = 50;
    let center_y = 50;

    flor(&mut framebuffer, center_x, center_y);

    loaf(&mut framebuffer, 15, 15);
    loaf(&mut framebuffer, 80, 15);
    loaf(&mut framebuffer, 15, 80);
    loaf(&mut framebuffer, 80, 80);

    toad(&mut framebuffer, 42, 25);
    toad(&mut framebuffer, 54, 25);
    toad(&mut framebuffer, 42, 75);
    toad(&mut framebuffer, 54, 75);

    glider(&mut framebuffer, 25, 25);
    glider(&mut framebuffer, 70, 25);
    glider(&mut framebuffer, 25, 70);
    glider(&mut framebuffer, 70, 70);

    blinker(&mut framebuffer, 25, 50);
    blinker(&mut framebuffer, 72, 50);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        actualizar_celulas(&mut framebuffer);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}

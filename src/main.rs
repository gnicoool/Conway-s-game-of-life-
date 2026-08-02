use minifb::{Key, Window, WindowOptions};

use std::time::Duration;

mod framebuffer;
mod conway;
mod organismos;

use crate::framebuffer::Framebuffer;
use crate::conway::actualizar_celulas;
use crate::organismos::{blinker, glider, toad};

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 80;
    let framebuffer_height = 60;

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

    let frame_delay = Duration::from_millis(30);

    blinker(&mut framebuffer, 10, 10);
    glider(&mut framebuffer, 20, 20);
    toad(&mut framebuffer, 30, 30);


    while window.is_open() && !window.is_key_down(Key::Escape) {
        actualizar_celulas(&mut framebuffer);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}

use crate::framebuffer::Framebuffer;

pub fn blinker(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x, y, true);
    fb.set_alive(x + 1, y, true);
    fb.set_alive(x + 2, y, true);
}

pub fn glider(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x + 1, y, true);
    fb.set_alive(x + 2, y + 1, true);
    fb.set_alive(x, y + 2, true);
    fb.set_alive(x + 1, y + 2, true);
    fb.set_alive(x + 2, y + 2, true);
} 

pub fn toad(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x + 1, y, true);
    fb.set_alive(x + 2, y, true);
    fb.set_alive(x + 3, y, true);
    fb.set_alive(x, y + 1, true);
    fb.set_alive(x + 1, y + 1, true);
    fb.set_alive(x + 2, y + 1, true);
}

pub fn loaf(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x + 1, y, true);
    fb.set_alive(x + 2, y, true);
    fb.set_alive(x, y + 1, true);
    fb.set_alive(x + 3, y + 1, true);
    fb.set_alive(x + 1, y + 2, true);
    fb.set_alive(x + 3, y + 2, true);
    fb.set_alive(x + 2, y + 3, true);
}
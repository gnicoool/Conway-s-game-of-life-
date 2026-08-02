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

pub fn flor(fb: &mut Framebuffer, x: usize, y: usize) {
    const OCTAVO: &[(i32, i32)] = &[
        // diagonales
        (0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7), (8, 8),
        
        // flor central
        (2, 0), (3, 0), (3, 1), (4, 0), (4, 1), (4, 2), (5, 1), (5, 2), (5, 3),
        (2, 1), (3, 2), (4, 3), (5, 4), (6, 5), (7, 6),

        (7, 0), (8, 0), (9, 0), (10, 0), (11, 0),
        (8, 1), (9, 1), (10, 1), (11, 1),
        (9, 2), (10, 2), (11, 2),
        (10, 3), (11, 3), (11, 4),
        (7, 1), (8, 2), (9, 3), (10, 4), (11, 5),
        (10, 5), (11, 6), (11, 7),
        (11, 8), (11, 9), (11, 10), (11, 11),
        (10, 11), (9, 11), (8, 11), (7, 11),
        (6, 11), (5, 11), (4, 11), (3, 11), (2, 11), (1, 11),
        (0, 11), (0, 10), (0, 9), (0, 8), (0, 7),
    ];

    // Aplicar simetría
    for &(dx, dy) in OCTAVO {
        for &(fx, fy) in &[(dx, dy), (dy, dx)] {
            for sx in [-1, 1] {
                for sy in [-1, 1] {
                    marcar(fb, x, y, fx * sx, fy * sy);
                }
            }
        }
    }
}

fn marcar(fb: &mut Framebuffer, x: usize, y: usize, dx: i32, dy: i32) {
    if let (Ok(px), Ok(py)) = (
        usize::try_from(x as i32 + dx),
        usize::try_from(y as i32 + dy),
    ) {
        fb.set_alive(px, py, true);
    }
}
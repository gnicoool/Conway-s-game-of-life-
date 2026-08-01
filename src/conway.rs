use crate::framebuffer::Framebuffer;

//uso de rem_euclid para resultados positivos y asi manejar los bordes como si estuvieran conectados 
pub fn contar_vivos(fb:&Framebuffer, x:usize, y:usize) -> u32 {
    let mut vivos = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let nx = (x as isize + i).rem_euclid(fb.width as isize) as usize;
            let ny = (y as isize + j).rem_euclid(fb.height as isize) as usize;
            if fb.is_alive(nx, ny) {
                vivos += 1;
            }
        }
    }
    vivos
}


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

pub fn actualizar_celulas(fb:&mut Framebuffer){
    let mut nuevas_celulas: Vec<bool> = vec![false; fb.width * fb.height];
    for y in 0..fb.height {
        for x in 0..fb.width {
            let vivos = contar_vivos(fb, x, y);
            if fb.is_alive(x, y) {
                // Celula viva sobrevive con 2 o 3 vecinos vivos; con menos o mas, muere.
                nuevas_celulas[y * fb.width + x] = vivos == 2 || vivos == 3;
            } else {
                // Celula muerta con exactamente 3 vecinos vivos nace.
                nuevas_celulas[y * fb.width + x] = vivos == 3;
            }
        }
    }
    for y in 0..fb.height {
        for x in 0..fb.width {
            fb.set_alive(x, y, nuevas_celulas[y * fb.width + x]);
        }
    }
}
pub fn contar_vivos(fb:&Framebuffer, x:usize, y:usize) -> u32 {
    let mut vivos = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let nx = x as isize + i;
            let ny = y as isize + j;
            if nx >= 0 && ny >= 0 && (nx as usize) < fb.width && (ny as usize) < fb.height {
                if fb.is_alive(nx as usize, ny as usize) {
                    vivos += 1;
                }
            }
        }
    }
    vivos
}
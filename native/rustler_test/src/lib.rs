
static MAX_ITERATIONS: u32 = 255;


fn xform(w: u32, h: u32, x: u32, y: u32, z: f32) -> (f32, f32) {
    let x0 = ((x as f32) / (w as f32)) * (4.0/z) - 2.0 / z;
    let y0 = ((y as f32) / (h as f32)) * (4.0/z) - 2.0 / z;
    (x0, y0)
}

fn get_color(i: u32, itermax: u32) -> (u8, u8, u8) {
    if i > itermax {
        return (255, 255, 255);
    }
    if itermax == 255 {
        let idx = i as u8;
        return (idx, idx, idx);
    }
    let idx = (((i as f32) / (itermax as f32)) * 255.0).round() as u8;
    (idx, idx, idx)
}

#[rustler::nif]
fn mandel_rgb(w: u32, h: u32, x: u32, y: u32, z: f32) -> Vec<u8> {
    let c: (f32, f32) = xform(w, h, x, y, z);
    let x0 = c.0;
    let y0 = c.1;
    let mut x = 0.0;
    let mut y = 0.0;
    let mut iteration: u32 = 0;
    while x * x + y * y <= 4.0 && iteration < MAX_ITERATIONS {
        let xtemp = x * x - y * y + x0;
        y = 2.0 * x * y + y0;
        x = xtemp;
        iteration = iteration + 1;
    }
    let rgb: (u8, u8, u8) = get_color(iteration, MAX_ITERATIONS);

    let mut v = Vec::new();
    v.push(rgb.0);
    v.push(rgb.1);
    v.push(rgb.2);  // convert to a list.
    v
}

rustler::init!("Elixir.RustlerTest.Native");

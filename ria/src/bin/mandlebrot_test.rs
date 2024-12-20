use image::{ImageBuffer, Rgb};

fn is_mandlebrot_point(point: (f64, f64), max_iters: u32) -> (bool, u32) {
    // z is a complex number
    let mut z = (0.0, 0.0);
    // calculating square of complex number
    let mut n: u32 = 0;
    while n < max_iters {
        let z_real = z.0 * z.0 - z.1 * z.1 + point.0;
        let z_img = z.0 * z.1 + point.1;
        z = (z_real, z_img);
        let magn = (z.0 * z.0 + z.1 * z.1).sqrt();
        if magn > 2.0 {
            return (false, n);
        }
        n += 1;
    }
    (true, n)
}

fn main() {
    let width = 800;
    let height = 800;
    let max_iters = 1000;
    let mut image_buff: ImageBuffer<Rgb<u8>, Vec<_>> = ImageBuffer::new(width, height);

    for (x, y, pixel_ref) in image_buff.enumerate_pixels_mut() {
        let cx = (x as f64 / width as f64) * 3.0 - 2.0;
        let cy = (y as f64 / height as f64) * 3.0 - 1.5;

        let (in_set, iters) = is_mandlebrot_point((cx, cy), max_iters);
        let color = if !in_set {
            let ratio = iters as f64 / max_iters as f64;
            ((1.0 - ratio) * 255.0) as u8
        } else {
            0
        };
        *pixel_ref = Rgb([color, color, color]);
    }
}

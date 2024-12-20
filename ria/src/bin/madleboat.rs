use image::{ImageBuffer, Rgb};

fn mandelbrot_point(c: (f64, f64), max_iters: u32) -> (bool, u32) {
    let mut z = (0.0, 0.0);
    let mut n = 0;

    while n < max_iters {
        let real = z.0 * z.0 - z.1 * z.1 + c.0; // z.real^2 - z.imag^2 + c.real
        let imag = 2.0 * z.0 * z.1 + c.1; // 2 * z.real * z.imag + c.imag
        z = (real, imag);

        // Check if the point has escaped
        if (z.0 * z.0 + z.1 * z.1).sqrt() > 2.0 {
            return (false, n); // Escaped, not part of the set
        }

        n += 1;
    }

    (true, n) // Did not escape, part of the set
}

fn main() {
    let img_width = 800;
    let img_height = 800;
    let max_iters = 1000;

    let mut imgbuf = ImageBuffer::new(img_width, img_height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = (x as f64 / img_width as f64) * 3.0 - 2.0; // Scale x to [-2.0, 1.0]
        let cy = (y as f64 / img_height as f64) * 3.0 - 1.5; // Scale y to [-1.5, 1.5]

        let (_in_set, iterations) = mandelbrot_point((cx, cy), max_iters);

        let color = if !_in_set {
            let ratio = iterations as f64 / max_iters as f64;
            (255.0 * (1.0 - ratio)) as u8 // Convert back to u8
        } else {
            0
        };

        *pixel = Rgb([color, color, color]);
    }

    imgbuf.save("mandelbrot_updated.png").unwrap();
    println!("Updated Mandelbrot set image saved as 'mandelbrot_updated.png'");
}

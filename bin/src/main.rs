use raytracer::{Color, io::write_color};
use std::io;

fn main() -> io::Result<()> {
    // Image
    const image_width: i32 = 256;
    const image_height: i32 = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    let mut stdout = std::io::stdout();
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let color = Color::from_rgb(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25
            );

            write_color(&mut stdout, color)?;
        }
    }
    eprintln!("\nDone.");
    Ok(())
}

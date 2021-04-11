use crate::{
    Canvas,
    Color,
    Point2,
};

pub fn write_ppm(
    writer: &mut dyn std::io::Write,
    canvas: & dyn Canvas,
    samples_per_pixel: usize
) -> std::io::Result<()> {
    println!("P3\n{} {}\n255", canvas.width(), canvas.height());
    for j in (0..canvas.height()).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..canvas.width() {
            write_color(writer, canvas.get(Point2 {x: i, y: j}), samples_per_pixel)?;
        }
    }
    Ok(()) 
}
pub fn write_color(
    writer: &mut dyn std::io::Write,
    color: &Color,
    samples_per_pixel: usize,
) -> std::io::Result<()> {
    let scale = 1.0/samples_per_pixel as f64;
    let r = (color.r() * scale).sqrt();
    let g = (color.g() * scale).sqrt();
    let b = (color.b() * scale).sqrt();
    writeln!(
        writer,
        "{} {} {}",
        (256. * clamp(r, 0., 0.999)) as i32,
        (256. * clamp(g, 0., 0.999)) as i32,
        (256. * clamp(b, 0., 0.999)) as i32
    )
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

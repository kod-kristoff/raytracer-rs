use crate::Color;

pub fn write_color(
    writer: &mut dyn std::io::Write,
    color: Color,
    samples_per_pixel: i32,
) -> std::io::Result<()> {
    let scale = 1.0/samples_per_pixel as f64;
    let color = scale*color;
    writeln!(
        writer,
        "{} {} {}",
        (256. * clamp(color.r(), 0., 0.999)) as i32,
        (256. * clamp(color.g(), 0., 0.999)) as i32,
        (256. * clamp(color.b(), 0., 0.999)) as i32
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

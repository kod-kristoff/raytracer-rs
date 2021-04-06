use crate::Color;

pub fn write_color(
    writer: &mut dyn std::io::Write,
    color: Color
) -> std::io::Result<()> {
    writeln!(
        writer,
        "{} {} {}",
        (255.999 * color.r()) as i32,
        (255.999 * color.g()) as i32,
        (255.999 * color.b()) as i32
    )
}

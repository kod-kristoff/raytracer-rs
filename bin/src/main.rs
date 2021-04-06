use raytracer::{
    Color, 
    io::write_color,
    Point,
    Ray,
    Vec3,
};
use std::io;

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Point::from_xyz(0., 0., 0.);
    let horizontal = Vec3::from_xyz(viewport_width, 0., 0.);
    let vertical = Vec3::from_xyz(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::from_xyz(0., 0., focal_length);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    let mut stdout = std::io::stdout();
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let color = ray_color(&ray);
            write_color(&mut stdout, color)?;
        }
    }
    eprintln!("\nDone.");
    Ok(())
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point::from_xyz(0., 0., -1.), 0.5, ray) {
        return Color::from_rgb(1., 0., 0.);
    }
    let unit_direction = ray.direction().to_unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    (1. - t)*Color::from_rgb(1., 1., 1.) + t*Color::from_rgb(0.5, 0.7, 1.)
}

fn hit_sphere(
    center: &Point,
    radius: f64,
    ray: &Ray
) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius*radius;
    let discriminant = b*b - 4.*a*c;
    discriminant > 0.0
}

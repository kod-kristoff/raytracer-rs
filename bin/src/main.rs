use raytracer::{
    Color, 
    HitRecord,
    Hittable,
    io::write_color,
    models::{List, Sphere},
    Point,
    Ray,
    Vec3,
};
use std::io;
use std::sync::Arc;

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    // World
    let world = create_scene();

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
            let color = ray_color(&ray, &world);
            write_color(&mut stdout, color)?;
        }
    }
    eprintln!("\nDone.");
    Ok(())
}

fn ray_color(ray: &Ray, world: &Box<dyn Hittable>) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(ray, 0., f64::INFINITY, &mut rec) {
        return 0.5*Color::from_rgb(rec.normal.x() + 1., rec.normal.y() + 1., rec.normal.z() + 1.);
        
    }
    let unit_direction = ray.direction().to_unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    (1. - t)*Color::from_rgb(1., 1., 1.) + t*Color::from_rgb(0.5, 0.7, 1.)
}

fn create_scene() -> Box<dyn Hittable> {
    let mut world = List::new();

    world.add(Arc::new(
        Sphere::new(Point::from_xyz(0., 0., -1.), 0.5)
    ));
    world.add(Arc::new(
        Sphere::new(Point::from_xyz(0., -100.5, -1.), 100.)
    ));

    Box::new(world)
}

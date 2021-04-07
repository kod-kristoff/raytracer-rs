use raytracer::{
    Camera,
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
    use rand::Rng;

    let mut rng = rand::thread_rng();
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // World
    let world = create_scene();

    // Camera
    let camera = Camera::default();

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    let mut stdout = std::io::stdout();
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut color = Color::black();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world);
            }
            write_color(&mut stdout, color, samples_per_pixel)?;
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

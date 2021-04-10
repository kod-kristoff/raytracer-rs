use raytracer::{
    Camera,
    Color, 
    Hittable,
    io::write_color,
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
    let max_depth = 50;

    // World
    let world = create_scene();

    // Camera
    let camera = Camera::new(
        Point::from_xyz(-2., 2., 1.),
        Point::from_xyz(0., 0., -1.),
        Vec3::from_xyz(0., 1., 0.),
        20.0, aspect_ratio);

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
                color += ray_color(&mut rng, &ray, &world, max_depth);
            }
            write_color(&mut stdout, color, samples_per_pixel)?;
        }
    }
    eprintln!("\nDone.");
    Ok(())
}

fn ray_color(rng: &mut dyn rand::RngCore, ray: &Ray, world: &Box<dyn Hittable>, depth: i32) -> Color {

    if depth <= 0 {
        return Color::black();
    }
    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        let material = rec.material.clone();
        if let Some((attenuation, scattered)) = material.and_then(|mat| mat.scatter(rng, ray, &rec)) {
            return attenuation * ray_color(rng, &scattered, world, depth-1);
        }
        return Color::black();
        // let target = rec.p + rec.normal + Vec3::random_unit_vector(rng);
        // return 0.5*ray_color(rng, &Ray::new(rec.p, target-rec.p), world, depth-1);
        // return 0.5*Color::from_rgb(rec.normal.x() + 1., rec.normal.y() + 1., rec.normal.z() + 1.);
        
    }
    let unit_direction = ray.direction().to_unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    (1. - t)*Color::from_rgb(1., 1., 1.) + t*Color::from_rgb(0.5, 0.7, 1.)
}

fn camera_scene() -> Box<dyn Hittable> {
    use raytracer::{
        models::{List, Sphere},
        materials::{Lambertian, Metal, Dielectric},
    };
    let mut world = List::new();

    let material_left = Arc::new(
        Lambertian::new(Color::from_rgb(0., 0., 1.0))
    );
    let material_right = Arc::new(
        Lambertian::new(Color::from_rgb(1.0, 0., 0.))
    );

    let r = (std::f64::consts::PI / 4.0).cos();
    world.add(Arc::new(
        Sphere::new(
            Point::from_xyz(-r, 0.0, -1.0),
            r,
            material_left
        )
    ));
    world.add(Arc::new(
        Sphere::new(
            Point::from_xyz(r, 0.0, -1.0),
            r,
            material_right
        )
    ));

    Box::new(world)
    
}

fn create_scene() -> Box<dyn Hittable> {
    use raytracer::{
        models::{List, Sphere},
        materials::{Lambertian, Metal, Dielectric},
    };
    let mut world = List::new();

    let material_ground = Arc::new(
        Lambertian::new(Color::from_rgb(0.8, 0.8, 0.0))
    );
    let material_center = Arc::new(
        Lambertian::new(Color::from_rgb(0.1, 0.2, 0.5))
    );
    // let material_left = Arc::new(
    //     Metal::new(Color::from_rgb(0.8, 0.8, 0.8), 0.3)
    // );
    let material_right = Arc::new(
        Metal::new(Color::from_rgb(0.8, 0.6, 0.2), 1.0)
    );
    let material_left = Arc::new(
        Dielectric::new(Color::from_rgb(0.8, 0.8, 0.8), 1.5)
    );
    // let material_center = Arc::new(
    //     Dielectric::new(Color::from_rgb(0.8, 0.6, 0.2), 1.5)
    // );

    world.add(Arc::new(
        Sphere::new(
            Point::from_xyz(0., -100.5, -1.), 
            100.0,
            material_ground
        )
    ));

    world.add(Arc::new(
        Sphere::new(
            Point::from_xyz(0.0, 0.0, -1.0),
            0.5,
            material_center
        )
    ));
    world.add(Arc::new(
        Sphere::new(
            Point::from_xyz(-1.0, 0.0, -1.0),
            0.5,
            material_left.clone()
        )
    ));
    world.add(Arc::new(
        Sphere::new(
            Point::from_xyz(-1.0, 0.0, -1.0),
            -0.45,
            material_left
        )
    ));
    world.add(Arc::new(
        Sphere::new(
            Point::from_xyz(1.0, 0.0, -1.0),
            0.5,
            material_right
        )
    ));

    Box::new(world)
}

use raytracer::{
    canvas,
    io::write_ppm,
    Renderer,
    Camera,
    Color, 
    Hittable,
    Point,
    Ray,
    Vec3,
};
use std::io;
use std::sync::Arc;

fn main() -> io::Result<()> {

    let mut rng = rand::thread_rng();
    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width = 400; //1200;
    let image_height = (image_width as f64 / aspect_ratio) as _;
    let samples_per_pixel = 100; //500;
    let max_depth = 50;

    // World
    let world = random_scene(&mut rng);

    // Camera
    let look_from = Point::from_xyz(13., 2., 3.);
    let look_at = Point::from_xyz(0., 0., 0.);
    let view_up = Vec3::from_xyz(0., 1., 0.);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        20.0, aspect_ratio,
        aperture,
        dist_to_focus
    );

    // Render
    let renderer = Renderer::new(samples_per_pixel, max_depth);
    let mut canvas = canvas::Grid::new(image_width, image_height);

    renderer.render(&mut rng, world, &camera, &mut canvas);


    let mut stdout = std::io::stdout();
    write_ppm(&mut stdout, &canvas)?;
    eprintln!("\nDone.");
    Ok(())
}

fn ray_color(rng: &mut dyn rand::RngCore, ray: &Ray, world: &Arc<dyn Hittable>, depth: i32) -> Color {

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

fn camera_scene() -> Arc<dyn Hittable> {
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

    Arc::new(world)
    
}

fn create_scene() -> Arc<dyn Hittable> {
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

    Arc::new(world)
}

fn random_scene(rng: &mut dyn rand::RngCore) -> Arc<dyn Hittable> {
    use raytracer::{
        Material,
        materials::{Dielectric, Lambertian, Metal},
        models::{List, Sphere},
        utils,
    };
    use rand::Rng;

    let mut world = List::new();

    let material_ground = Arc::new(
        Lambertian::new(Color::from_rgb(0.5, 0.5, 0.5))
    );
    world.add(Arc::new(Sphere::new(
        Point::from_xyz(0.,-1000.,0.), 1000.,
        material_ground
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point::from_xyz(
                a as f64 + 0.9*rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9*rng.gen::<f64>()
            );

            if (center - Point::from_xyz(4., 0.2,0.)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(rng) * Color::random(rng);
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in_interval(rng, 0.5, 1.);
                    let fuzz = utils::random_in_interval(rng, 0., 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(Color::white(), 1.5))
                };
                world.add(Arc::new(Sphere::new(
                    center, 0.2, sphere_material
                )));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(Color::white(), 1.5));
    world.add(Arc::new(Sphere::new(
        Point::from_xyz(0.,1.,0.), 1., material1)));
    let material2 = Arc::new(Lambertian::new(Color::from_rgb(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point::from_xyz(-4.,1.,0.), 1., material2)));
    let material3 = Arc::new(Metal::new(Color::from_rgb(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point::from_xyz(4.,1.,0.), 1., material3)));

    Arc::new(world)
}

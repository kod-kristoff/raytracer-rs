use crate::{
    Camera,
    Canvas,
    Color,
    Hittable,
    Point2,
    Ray,
};
use std::sync::Arc;

pub struct Renderer {
    samples_per_pixel: usize,
    max_depth: i32,
}

impl Renderer {
    pub fn new(samples_per_pixel: usize, max_depth: i32) -> Self {
        Self { samples_per_pixel, max_depth }
    }

    pub fn render(
        &self,
        rng: &mut dyn rand::RngCore,
        world: Arc<dyn Hittable>,
        camera: &Camera,
        canvas: &mut dyn Canvas,
    ) {
        use rand::Rng;

        for j in 0..canvas.height() {
            eprint!("\rScanline: {}/{} ", j, canvas.height());
            for i in 0..canvas.width() {
                let mut color = Color::black();
                for _ in 0..self.samples_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) / (canvas.width() - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (canvas.height() - 1) as f64;

                    let ray = camera.get_ray(rng, u, v);
                    color += ray_color(rng, &ray, &world, self.max_depth);
                }
                canvas.set(Point2 {x: i, y:j}, color);
            }
        }
        
    }
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

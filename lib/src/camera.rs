use crate::{Point, Ray, Vec3};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let origin = Point::from_xyz(0., 0., 0.);
        let horizontal = Vec3::from_xyz(viewport_width, 0., 0.);
        let vertical = Vec3::from_xyz(0., viewport_height, 0.);
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::from_xyz(0., 0., focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin, 
            self.lower_left_corner 
                + u*self.horizontal 
                + v*self.vertical 
                - self.origin
        )

    }
}
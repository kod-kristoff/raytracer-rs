use crate::{
    Point,
    Ray,
    Vec3,
    cross,
};

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
    pub fn new(
        look_from: Point,
        look_at: Point,
        view_up: Vec3,
        vert_fov: f64,
        aspect_ratio: f64
    ) -> Self {
        let theta = vert_fov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).to_unit_vector();
        let u = cross(&view_up, &w).to_unit_vector();
        let v = cross(&w, &u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

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

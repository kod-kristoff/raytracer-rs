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
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
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
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            lens_radius: 0.0
        }
    }
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        view_up: Vec3,
        vert_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vert_fov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).to_unit_vector();
        let u = cross(&view_up, &w).to_unit_vector();
        let v = cross(&w, &u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist*w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u, v, w,
            lens_radius: aperture/2.0
        }
    }

    pub fn get_ray(&self, rng: &mut dyn rand::RngCore, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = rd.x()*self.u + rd.y()*self.v;
        Ray::new(
            self.origin + offset, 
            self.lower_left_corner 
                + s*self.horizontal 
                + t*self.vertical 
                - self.origin
                - offset
        )

    }
}

use crate::{
    ray::Ray,
    utils::degrees_to_radians,
    vec3::{Point3, Vec3},
};
use rand::{thread_rng, Rng};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    shutter_open_time: f64,
    shutter_close_time: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3,
        vertical_field_of_view_in_degrees: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
        shutter_open_time: f64,
        shutter_close_time: f64,
    ) -> Self {
        let theta = degrees_to_radians(vertical_field_of_view_in_degrees);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = view_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w,
            u,
            v,
            lens_radius: aperture / 2.0,
            shutter_open_time,
            shutter_close_time,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            thread_rng().gen_range(self.shutter_open_time..=self.shutter_close_time),
        )
    }
}

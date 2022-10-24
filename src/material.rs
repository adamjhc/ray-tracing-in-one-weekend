use crate::{
    colour::Colour,
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColour, Texture},
    vec3::{Point3, Vec3},
};
use rand::random;
use std::sync::Arc;

pub trait Material: Sync + Send {
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Colour {
        Colour::default()
    }

    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Colour, Ray)>;
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn from(albedo: Colour) -> Self {
        Self::new(Arc::new(SolidColour::from(albedo)))
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some((
            self.albedo.value(hit_record.u, hit_record.v, hit_record.p),
            Ray::new(hit_record.p, scatter_direction, ray_in.time),
        ))
    }
}

pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected = ray_in.direction.unit_vector().reflect(&hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            ray_in.time,
        );

        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Colour, Ray)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
                unit_direction.reflect(&hit_record.normal)
            } else {
                unit_direction.refract(&hit_record.normal, refraction_ratio)
            };

        Some((
            Colour::new(1.0, 1.0, 1.0),
            Ray::new(hit_record.p, direction, ray_in.time),
        ))
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(colour: Colour) -> Self {
        Self {
            emit: Arc::new(SolidColour::from(colour)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Colour {
        self.emit.value(u, v, *p)
    }

    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Colour, Ray)> {
        None
    }
}

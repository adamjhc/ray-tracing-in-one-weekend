use crate::{colour::Colour, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray::new(hit_record.p, scatter_direction);
        *attenuation = self.albedo;
        true
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
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.direction.unit_vector().reflect(&hit_record.normal);
        *scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        *attenuation = self.albedo;
        scattered.direction.dot(&hit_record.normal) > 0.0
    }
}

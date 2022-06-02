use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

impl Material for () {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut Color, _: &mut Ray) -> bool {
        false
    }
}

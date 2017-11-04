use material::{Material, MaterialResult};
use vec3::{Vec3, dot};
use ray::Ray;
use hitable::HitRecord;

extern crate rand;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian{
            albedo: albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<MaterialResult> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        Some(MaterialResult::new(self.albedo.clone(), Ray::new(rec.p, target - rec.p)))
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) - Vec3::new(1.0, 1.0, 1.0);
        if dot(&p, &p) >= 1.0 {
            return p;
        }
    }
}

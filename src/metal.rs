use material::{Material, MaterialResult};
use vec3::{Vec3, dot};
use ray::Ray;
use hitable::HitRecord;

#[derive(Clone)]
pub struct Metal {
    albedo: Vec3
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal{
            albedo: albedo
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<MaterialResult> {
        let reflected = reflect(ray_in.direction(), &rec.normal);
        let res = MaterialResult::new(self.albedo.clone(), Ray::new(rec.p, reflected));
        if dot(&res.scattered.direction(), &rec.normal) > 0.0 {
            return Some(res);
        }
        None
    }
}

fn reflect(v: Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * dot(&v, &n) * *n
}

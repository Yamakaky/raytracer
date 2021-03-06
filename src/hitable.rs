use vec3::Vec3;
use ray::Ray;
use material::Material;

use std::rc::Rc;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Rc<Material>
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3, mat: Rc<Material>) -> HitRecord {
        HitRecord{
            t: t,
            p: p,
            normal: normal,
            mat: mat
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

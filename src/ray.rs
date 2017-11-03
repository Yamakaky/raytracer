use vec3::{Vec3, unit_vector};


pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray{
            origin: origin,
            direction: direction
        }
    }

    pub fn direction(&self) -> Vec3 {
        unit_vector(&self.direction)
    }
}

#[test]
fn ray_point_at_parameter() {
    let r = Ray::new(Default::default(), Vec3::new(1.0, 0.0, 0.0));
    let p = r.point_at_parameter(2.0);
    assert!(p == Vec3::new(2.0, 0.0, 0.0));
}

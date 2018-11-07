use na::Vector3;
use na::Dot;
use ray::Ray;
use hitrecord::HitRecord;
use hitable::Hitable;
use material::Material;


pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
	    let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut t = (-b - discriminant.sqrt())/a;
            if t < t_max && t > t_min {
                let point = r.point_at(t);
                return Some(HitRecord{
                    t: t,
                    p: point,
                    normal: (point - self.center) / self.radius,
                    material: self.material.as_ref(),
                });
            }
            t = (-b + discriminant.sqrt())/a;
            if t < t_max && t > t_min {
                let point = r.point_at(t);
                return Some(HitRecord{
                    t: t,
                    p: point,
                    normal: (point - self.center) / self.radius,
                    material: self.material.as_ref()
                });
            }
        }
        return None;
    }
}

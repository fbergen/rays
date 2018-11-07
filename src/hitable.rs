use hitrecord::HitRecord;
use ray::Ray;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min:f64, t_max:f64) -> Option<HitRecord>;
}

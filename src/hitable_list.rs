use ray::Ray;
use hitrecord::HitRecord;
use hitable::Hitable;

pub struct HitableList {
    pub list: Vec<Box<Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_dist = t_max;
        for x in &self.list {
            let ret = x.hit(r, t_min, closest_dist);
            if ret.is_some() {
                hit_record = ret;
                closest_dist = hit_record.as_ref().map(|hr| hr.t).unwrap();
            }
        }
        hit_record
    }
}

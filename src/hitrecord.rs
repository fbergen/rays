use na::Vector3;
use material::Material;

pub struct HitRecord<'a>{
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: &'a Material,
}

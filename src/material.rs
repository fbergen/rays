
use rand::prelude::*;
use na::Norm;
use na::dot;
use na::Dot;
use hitrecord::HitRecord;
use na::Vector3;
use ray::Ray;

pub struct ScatterRec {
    pub attenuation: Vector3<f64>,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRec>;
}

pub struct Lambertian {
    pub albedo: Vector3<f64>
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRec> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        Some(ScatterRec{
            attenuation: self.albedo,
            scattered: Ray{
                origin: rec.p,
                direction: target - rec.p
            }
        })
    }
}

pub struct Metal {
    pub fuzz: f64,
    pub albedo: Vector3<f64>
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRec> {
        let reflected= reflect(&r_in.direction.normalize(), &rec.normal);
        let scattered = Ray{origin: rec.p, direction: reflected + self.fuzz*random_in_unit_sphere()};
        if scattered.direction.dot(&rec.normal) > 0.0 {
            return Some(ScatterRec{
                scattered: scattered,
                attenuation: self.albedo,
            })
        }
        None
    }
    
}

pub struct Dielectric {
    pub ref_idx:f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRec> {
        let mut ni_over_nt = self.ref_idx;
        let cosine;
        // Reflect everything.
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let drn = r_in.direction.dot(&rec.normal);
        let outward_normal = match drn > 0.0 {
            true => { 
                cosine = self.ref_idx * drn / r_in.direction.norm_squared().sqrt();
                -rec.normal
            },
            false => { 
                cosine = -drn / r_in.direction.norm_squared().sqrt();
                ni_over_nt = 1.0 / self.ref_idx; 
                rec.normal
            }
        };

        match refract(&r_in.direction, &outward_normal, ni_over_nt) {
	        Some(refraction) => { 
                let reflect_prob = schlick(cosine, self.ref_idx);
	            if rand::random::<f64>() >= reflect_prob {
	                return Some(ScatterRec{ 
                        attenuation: attenuation,
                        scattered: Ray{
                            origin: rec.p,
                            direction: refraction
                        }
	                })
                }
	        },
	        None => { }
	    }

        // Reflect.
        let reflected = reflect(&r_in.direction, &rec.normal);
        Some(ScatterRec{
            attenuation: attenuation,
            scattered: Ray{
                origin: rec.p,
                direction: reflected},
        })
    }
}

fn refract(v:&Vector3<f64>, n:&Vector3<f64>, ni_over_nt:f64) -> Option<Vector3<f64>> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt*dt);
    if discriminant > 0.0 {
        return Some(ni_over_nt*(uv - n*dt) - discriminant.sqrt() * n)
    }
    None
}

fn schlick(cosine:f64, ref_idx:f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0-r0) * (1.0 - cosine).powi(5)
}

fn random_in_unit_sphere() -> Vector3<f64> {
    let ones = Vector3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vector3::new(random::<f64>(), random::<f64>(), random::<f64>()) - ones;
        if p.norm_squared() < 1.0 {
            return p
        }
    }
}

fn reflect(v:&Vector3<f64>, n:&Vector3<f64>) -> Vector3<f64> {
    v - 2.0*dot(v,n)*n
}



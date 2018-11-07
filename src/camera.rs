use na::Vector3;
use na::Dot;
use ray::Ray;
use std::f64;
use na::Norm;
use na::Cross;

pub struct Camera {
  pub origin: Vector3<f64>,
  pub lower_left_corner: Vector3<f64>,
  pub horizontal: Vector3<f64>,
  pub vertical: Vector3<f64>,
  pub lens_radius: f64,
  u: Vector3<f64>,
  v: Vector3<f64>,
}

impl Camera {
  pub fn new(lookfrom: Vector3<f64>, lookat: Vector3<f64>, vup: Vector3<f64>, vfov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
    let theta = vfov * f64::consts::PI / 180.0;
    let half_height = f64::tan(theta / 2.0);
    let half_width = aspect * half_height;    

    let w = (lookfrom - lookat).normalize();
    let u = vup.cross(&w).normalize();
    let v = w.cross(&u);

    let lower_left_corner = lookfrom - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w; 
    let horizontal = 2.0 * half_width * focus_dist * u;
    let vertical = 2.0 * half_height * focus_dist * v;
    let lens_radius =  aperture / 2.0;
    let origin = lookfrom;

    Camera {lower_left_corner, horizontal, vertical, origin, lens_radius, u, v}
  }

  pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    let rd = self.lens_radius * random_in_unit_disk();
    let offset = rd[0] * self.u + rd[1] * self.v;

    Ray{origin: self.origin + offset,
		direction: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset}
  }
}

fn random_in_unit_disk() -> Vector3<f64> {
  loop {
    let p = 2.0 * Vector3::new(rand::random::<f64>(), rand::random::<f64>(), 0.0) - Vector3::new(1.0, 1.0, 0.0);
    if p.dot(&p) < 1.0 {
      return p;
    }
  }
}

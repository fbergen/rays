extern crate nalgebra as na;
extern crate rand; 
mod ray;
mod hitrecord;
mod hitable;
mod hitable_list;
mod sphere;
mod camera;
mod material;

use ray::Ray;
use na::Norm;    
use na::Vector3;
use na::Unit;
use std::f64;
use hitable::Hitable;
use hitable_list::HitableList;
use sphere::Sphere;
use camera::Camera;
use rand::prelude::*;
use material::Metal;
use material::Lambertian;
use material::Dielectric;

fn random_scene() -> HitableList {
	let mut hitable_list:Vec<Box<dyn Hitable>> = vec![Box::new(Sphere{
        center: Vector3::new(0.0, -1000.0, 0.0), 
        radius: 1000.0, 
        material: Box::new(Lambertian{albedo: Vector3::new(0.5, 0.5, 0.5)})
    })];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Vector3::new((a as f64) + 0.9 * random::<f64>(), 0.2, (b as f64) + 0.9 * random::<f64>());
            if (center - Vector3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {  // diffuse                   
                    hitable_list.push(Box::new(Sphere{
                        center: center,
                        radius: 0.2,
                        material: Box::new(Lambertian{
                            albedo: Vector3::new(random::<f64>()*random::<f64>(),
                                                 random::<f64>()*random::<f64>(),
                                                 random::<f64>()*random::<f64>())
                        })
                    }));
                } else if choose_mat < 0.95 { // metal
                    hitable_list.push(Box::new(Sphere{
                        center: center,
                        radius: 0.2,
                        material: Box::new(Metal{
                            fuzz: 0.5 * random::<f64>(),
                            albedo: Vector3::new(
                                0.5 * (1.0 + random::<f64>()),
                                0.5 * (1.0 + random::<f64>()),
                                0.5 * (1.0 + random::<f64>())),
                        })
                    }));
                } else { // glass
                    hitable_list.push(Box::new(Sphere{
                        center: center,
                        radius: 0.2,
                        material: Box::new(Dielectric{ref_idx: 1.5})
                    }));
                }
            }
        }
    }

    hitable_list.push(Box::new(Sphere{
        center: Vector3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric{ref_idx: 1.5})
    }));
    hitable_list.push(Box::new(Sphere{
        center: Vector3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian{
            albedo: Vector3::new(0.4, 0.2, 0.1)})
    }));
    hitable_list.push(Box::new(Sphere{
        center: Vector3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal{
            fuzz: 0.0,
            albedo: Vector3::new(0.7, 0.6, 0.5),
        })
    }));
 
    HitableList{list: hitable_list}
}

fn color(ray:&Ray, world:&dyn Hitable, depth:i32) -> Vector3<f64> {
    let rec = world.hit(ray, 0.001, f64::MAX);
    match rec {
        Some(rec_val) => {
            if depth < 50 {
                match rec_val.material.scatter(ray, &rec_val) {
                    Some(s) => {
                        return s.attenuation * color(&s.scattered, world, depth+1)
                    },
                    None => {}
                }
            }
            return Vector3::new(0.0, 0.0, 0.0)
        },
        None => {}
    }
    let u = Unit::new(&ray.direction);
    let t = 0.5 * (u.as_ref()[1] + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)

}

//    let hitlist = HitableList{
//        list: vec![Box::new(Sphere{center: Vector3::new(0.0, 0.0, -1.0), radius: 0.5, material: Box::new(Lambertian{albedo: Vector3::new(0.1, 0.2, 0.5)})}),
//                   Box::new(Sphere{center: Vector3::new(0.0, -100.5, -1.0), radius: 100.0, material: Box::new(Lambertian{albedo: Vector3::new(0.8, 0.8, 0.0)})}),
//                   Box::new(Sphere{center: Vector3::new(1.0, 0.0, -1.0), radius: 0.5, material: Box::new(Metal{fuzz: 1.0, albedo: Vector3::new(0.8, 0.6, 0.2)})}),
//                   Box::new(Sphere{center: Vector3::new(-1.0, 0.0, -1.0), radius: 0.5, material: Box::new(Dielectric{ref_idx: 1.5})}),
//                   Box::new(Sphere{center: Vector3::new(-1.0, 0.0, -1.0), radius: -0.45, material: Box::new(Dielectric{ref_idx: 1.5})})]
//    };
 

fn main() {
    let size_mul = 8;
    let (nx, ny) = (100 * size_mul, 50 * size_mul);
    let ns = 100;
    let max = 255;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("{}", max);
	let lookfrom = Vector3::new(6.0, 1.2, 2.5);
	let lookat = Vector3::new(0.0, 0.0, -1.0);
	let dist_to_focus = (lookfrom - lookat).norm();
	let aperture = 0.0;

	let camera = Camera::new(lookfrom, lookat, Vector3::new(0.0, 1.0, 0.0), 40.0, (nx as f64) / (ny as f64), aperture, dist_to_focus);

    let hitlist = random_scene();

    //let r = f64::cos(f64::consts::PI/4.0);
    //let hitlist = HitableList{
    //    list: vec![Rc::new(Sphere{center: Vector3::new(-r, 0.0, -1.0), radius: r, material: Box::new(Lambertian{albedo: Vector3::new(0.0, 0.0, 1.0)})}),
    //              Rc::new(Sphere{center: Vector3::new(r, 0.0, -1.0), radius: r, material: Box::new(Lambertian{albedo: Vector3::new(1.0, 0.0, 0.0)})})]
    //};
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f64 + random::<f64>()) / nx as f64;
                let v = (j as f64 + random::<f64>()) / ny as f64;
                let r = camera.get_ray(u, v);
                col += color(&r, &hitlist, 0);
            }
            col *= 1.0 / ns as f64;
            let c = Vector3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
            let ir = (255.99*c[0]) as i32;
            let ig = (255.99*c[1]) as i32;
            let ib = (255.99*c[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}


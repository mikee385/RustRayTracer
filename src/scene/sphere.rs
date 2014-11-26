use std::num::{Float};

use geometry::{Point3D, Vector3D, Direction3D, Ray3D};
use material::{Material};

use super::scene_object::{SceneObject};

#[deriving(PartialEq, PartialOrd, Clone, Show)]
pub struct Sphere {
    center: Point3D,
    radius: f32,
    radius_squared: f32,
    material: Material
}

impl Sphere {
    pub fn new(center: &Point3D, radius: f32, material: &Material) -> Sphere {
        Sphere {
            center: *center,
            radius: radius,
            radius_squared: radius * radius,
            material: *material
        }
    }
    
    pub fn get_center(&self) -> &Point3D {
        &self.center
    }
    
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}

impl SceneObject for Sphere {
    fn intersect(&self, ray: &Ray3D) -> Option<f32> {
        let sphere_to_ray = Vector3D::between_points(&ray.origin, &self.center);
        let b = Vector3D::dot(&sphere_to_ray, ray.direction.as_vector());
        if b < 0.0 {
            return None;
        }

        let d_squared = Vector3D::dot(&sphere_to_ray, &sphere_to_ray) - b * b;
        if d_squared > self.radius_squared {
            return None;
        }

        let c = (self.radius_squared - d_squared).sqrt();
        let mut t = b - c;
        if t < 0.0 {
            t = b + c;
        }

        Some(t)
    }
    
    fn get_normal(&self, point: &Point3D) -> Direction3D {
        Direction3D::between_points(&self.center, point)
    }
    
    fn get_material(&self, _: &Point3D) -> Material {
        self.material
    }
}

use std::num::{Float};

use geometry::{EPSILON, Point3D, Vector3D, AsVector, Direction3D, Ray3D};
use material::{Material};

use super::scene_object::{SceneObject};

#[derive(Clone, Debug)]
pub struct Plane {
    origin: Point3D,
    normal: Direction3D,
    material: Material
}

impl Plane {
    pub fn from_origin_normal(origin: &Point3D, normal: &Direction3D, material: &Material) -> Plane {
        Plane {
            origin: origin.clone(),
            normal: normal.clone(),
            material: material.clone()
        }
    }
    
    pub fn from_d_vector<T: AsVector>(d: f32, vector: &T, material: &Material) -> Plane {
        let v = vector.as_vector();        
        Plane {
            origin: Point3D::from_vector(&(v * (-d / Vector3D::dot(v, v)))),
            normal: Direction3D::from_vector(v),
            material: material.clone()
        }
    }
    
    pub fn get_origin(&self) -> &Point3D {
        &self.origin
    }
    
    pub fn get_d(&self) -> f32 {
        -Vector3D::dot(&Vector3D::from_point(&self.origin), &self.normal)
    }
}

impl SceneObject for Plane {
    fn intersect(&self, ray: &Ray3D) -> Option<f32> {
        let denominator = Vector3D::dot(&ray.direction, &self.normal);
        if denominator.abs() < EPSILON {
            return None;
        }

        let t = Vector3D::dot(&Vector3D::between_points(&ray.origin, &self.origin), &self.normal) / denominator;
        if t < 0.0 {
            return None;
        }

        Some(t)
    }
    
    fn get_normal(&self, _: &Point3D) -> Direction3D {
        self.normal.clone()
    }
    
    fn get_material(&self, _: &Point3D) -> Material {
        self.material.clone()
    }
}

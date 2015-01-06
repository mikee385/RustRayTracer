use std::num::{Float};

use geometry::{EPSILON, Point3D, Vector3D, Direction3D, Ray3D};
use material::{Material};

use super::scene_object::{SceneObject};

#[derive(PartialEq, PartialOrd, Clone, Show)]
pub struct Plane {
    origin: Point3D,
    normal: Direction3D,
    material: Material
}

impl Plane {
    pub fn from_origin_normal(origin: &Point3D, normal: &Direction3D, material: &Material) -> Plane {
        Plane {
            origin: *origin,
            normal: *normal,
            material: *material
        }
    }
    
    pub fn from_d_vector(d: f32, vector: &Vector3D, material: &Material) -> Plane {
        Plane {
            origin: Point3D::from_vector(&(*vector * (-d / Vector3D::dot(vector, vector)))),
            normal: Direction3D::from_vector(vector),
            material: *material
        }
    }
    
    pub fn get_origin(&self) -> &Point3D {
        &self.origin
    }
    
    pub fn get_d(&self) -> f32 {
        -Vector3D::dot(&Vector3D::from_point(&self.origin), self.normal.as_vector())
    }
}

impl SceneObject for Plane {
    fn intersect(&self, ray: &Ray3D) -> Option<f32> {
        let denominator = Vector3D::dot(ray.direction.as_vector(), self.normal.as_vector());
        if denominator.abs() < EPSILON {
            return None;
        }

        let t = Vector3D::dot(&Vector3D::between_points(&ray.origin, &self.origin), self.normal.as_vector()) / denominator;
        if t < 0.0 {
            return None;
        }

        Some(t)
    }
    
    fn get_normal(&self, _: &Point3D) -> Direction3D {
        self.normal
    }
    
    fn get_material(&self, _: &Point3D) -> Material {
        self.material
    }
}

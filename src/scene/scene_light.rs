use geometry::{Point3D, Direction3D, Ray3D};
use color::{ColorRGB};
use material::{Material};

use super::scene_object::{SceneObject};
use super::sphere::{Sphere};

#[derive(Clone, Show)]
pub struct SceneLight {
    sphere: Sphere
}

impl SceneLight {
    pub fn new(center: &Point3D, radius: f32, color: &ColorRGB) -> SceneLight {
        SceneLight {
            sphere: Sphere::new(center, radius, &Material::new(color))
        }
    }
    
    pub fn get_center(&self) -> &Point3D {
        self.sphere.get_center()
    }
    
    pub fn get_radius(&self) -> f32 {
        self.sphere.get_radius()
    }
}

impl SceneObject for SceneLight {
    fn intersect(&self, ray: &Ray3D) -> Option<f32> {
        self.sphere.intersect(ray)
    }
    
    fn get_normal(&self, point: &Point3D) -> Direction3D {
        self.sphere.get_normal(point)
    }
    
    fn get_material(&self, point: &Point3D) -> Material {
        self.sphere.get_material(point)
    }
}

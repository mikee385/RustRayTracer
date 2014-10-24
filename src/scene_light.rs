#![allow(dead_code)]

use geometry::{Point3D};
use color::{ColorRGB};
use material::{Material};
use scene_object::{SceneObject};
use sphere::{Sphere};

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
    
    pub fn get_material(&self, point: &Point3D) -> Material {
        self.sphere.get_material(point)
    }
    
    pub fn as_scene_object(&self) -> &SceneObject {
        &self.sphere
    }
}
use geometry::{Point3D, Direction3D, Ray3D};
use material::{Material};

pub trait SceneObject {
    fn intersect(&self, ray: &Ray3D) -> Option<f32>;
    fn get_normal(&self, point: &Point3D) -> Direction3D;
    fn get_material(&self, point: &Point3D) -> Material;
}
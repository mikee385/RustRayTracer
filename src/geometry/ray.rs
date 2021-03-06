use super::{Point3D, Direction3D};

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Ray3D {
    pub origin: Point3D,
    pub direction: Direction3D
}

impl Ray3D {
    pub fn new(origin: &Point3D, direction: &Direction3D) -> Ray3D {
        Ray3D { 
            origin: origin.clone(),
            direction: direction.clone()
        }
    }
    
    pub fn point(&self, distance: f32) -> Point3D {
        self.origin.translate_dist(&self.direction, distance)
    }
}

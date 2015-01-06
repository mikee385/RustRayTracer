use super::{Point3D, Direction3D};

#[derive(PartialEq, PartialOrd, Copy, Clone, Show)]
pub struct Ray3D {
    pub origin: Point3D,
    pub direction: Direction3D
}

impl Ray3D {
    pub fn new(origin: &Point3D, direction: &Direction3D) -> Ray3D {
        Ray3D { 
            origin: *origin,
            direction: *direction
        }
    }
    
    pub fn point(&self, distance: f32) -> Point3D {
        self.origin.translate_dist(&self.direction, distance)
    }
}

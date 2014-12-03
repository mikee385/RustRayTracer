use std::f32::consts::{PI};

pub use self::point::{Point3D};
pub use self::vector::{Vector3D};
pub use self::direction::{Direction3D};
pub use self::ray::{Ray3D};
pub use self::matrix::{Matrix3D};

mod point;
mod vector;
mod direction;
mod ray;
mod matrix;

pub const EPSILON: f32 = 1.0E-9;
pub const DEGREES_TO_RADIANS: f32 = PI / 180.0;

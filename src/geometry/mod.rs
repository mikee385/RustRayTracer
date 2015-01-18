use std::f32::consts::{PI};

pub use self::point::{Point3D};
pub use self::vector::{Vector3D, AsVector};
pub use self::direction::{Direction3D};
pub use self::ray::{Ray3D};
pub use self::matrix::{Matrix3D};

#[macro_use]
pub mod vector;

pub mod point;
pub mod direction;
pub mod ray;
pub mod matrix;

pub const EPSILON: f32 = 1.0E-9;
pub const DEGREES_TO_RADIANS: f32 = PI / 180.0;

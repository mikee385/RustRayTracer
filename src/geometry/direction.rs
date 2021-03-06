use std::ops::{Add, Sub, Mul, Div, Neg};

use super::{Point3D, Vector3D, AsVector, Matrix3D};

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Direction3D {
    direction: Vector3D
}

static DIRECTION3D_UNIT_X: Direction3D = Direction3D {
    direction: Vector3D {x: 1.0, y: 0.0, z: 0.0}
};
static DIRECTION3D_UNIT_Y: Direction3D = Direction3D {
    direction: Vector3D {x: 0.0, y: 1.0, z: 0.0}
};
static DIRECTION3D_UNIT_Z: Direction3D = Direction3D {
    direction: Vector3D {x: 0.0, y: 0.0, z: 1.0}
};

impl Direction3D {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Direction3D {
        Direction3D::from_vector(&Vector3D::from_xyz(x, y, z))
    }
    
    pub fn between_points(from: &Point3D, to: &Point3D) -> Direction3D {
        Direction3D::from_vector(&Vector3D::between_points(from, to))
    }
    
    pub fn from_point(point: &Point3D) -> Direction3D {
        Direction3D::from_vector(&Vector3D::from_point(point))
    }
    
    pub fn from_vector<T: AsVector>(vector: &T) -> Direction3D {
        let v = vector.as_vector();
        let magnitude = v.magnitude();
        if magnitude > 0.0 {
            Direction3D::from_normalized_vector(&v.scale(1.0 / magnitude))
        } else {
            Direction3D::from_normalized_vector(v)
        }
    }
    
    fn from_normalized_vector(normalized_vector: &Vector3D) -> Direction3D {
        Direction3D {direction: normalized_vector.clone()}
    }
    
    pub fn unit_x() -> &'static Direction3D {
        &DIRECTION3D_UNIT_X
    }
    
    pub fn unit_y() -> &'static Direction3D {
        &DIRECTION3D_UNIT_Y
    }
    
    pub fn unit_z() -> &'static Direction3D {
        &DIRECTION3D_UNIT_Z
    }
    
    pub fn x(&self) -> f32 {
        self.direction.x
    }
    
    pub fn y(&self) -> f32 {
        self.direction.y
    }
    
    pub fn z(&self) -> f32 {
        self.direction.z
    }

    pub fn to_orthonormal_basis(&self) -> Matrix3D
    {
        if self.direction.x.abs() >= self.direction.y.abs() && self.direction.x.abs() >= self.direction.z.abs() {
            let unit_x = &self.direction;

            let inv_xy_magnitude = 1.0 / (unit_x.x * unit_x.x + unit_x.y * unit_x.y).sqrt();
            let unit_y = Vector3D::from_xyz(
                -unit_x.y * inv_xy_magnitude,
                 unit_x.x * inv_xy_magnitude,
                 0.0);

            let unit_z = Vector3D::from_xyz(
                -unit_x.z * unit_y.y,
                 unit_x.z * unit_y.x,
                 unit_x.x * unit_y.y - unit_x.y * unit_y.x);

            return Matrix3D::new(unit_x, &unit_y, &unit_z);
        } else if self.direction.y.abs() >= self.direction.z.abs() {
            let unit_y = &self.direction;

            let inv_yz_magnitude = 1.0 / (unit_y.y * unit_y.y + unit_y.z * unit_y.z).sqrt();
            let unit_z = Vector3D::from_xyz(
                 0.0,
                -unit_y.z * inv_yz_magnitude,
                 unit_y.y * inv_yz_magnitude);

            let unit_x = Vector3D::from_xyz(
                 unit_y.y * unit_z.z - unit_y.z * unit_z.y,
                -unit_y.x * unit_z.z,
                 unit_y.x * unit_z.y);

            return Matrix3D::new(&unit_x, unit_y, &unit_z);
        } else {
            let unit_z = &self.direction;

            let inv_zx_magnitude = 1.0 / (unit_z.z * unit_z.z + unit_z.x * unit_z.x).sqrt();
            let unit_x = Vector3D::from_xyz(
                 unit_z.z * inv_zx_magnitude,
                 0.0,
                -unit_z.x * inv_zx_magnitude);

            let unit_y = Vector3D::from_xyz(
                 unit_z.y * unit_x.z,
                 unit_z.z * unit_x.x - unit_z.x * unit_x.z,
                -unit_z.y * unit_x.x);

            return Matrix3D::new(&unit_x, &unit_y, unit_z);
        }
    }
    
    pub fn eq_tol<T: AsVector>(&self, other: &T, tolerance: f32) -> bool {
        let v = other.as_vector();
        (self.direction.x - v.x).abs() < tolerance &&
        (self.direction.y - v.z).abs() < tolerance &&
        (self.direction.y - v.z).abs() < tolerance
    }
}

//------------------------------------------------------------------------------

impl AsVector for Direction3D {
    fn as_vector<'a>(&'a self) -> &'a Vector3D {
        &self.direction
    }
}

as_vector_add!(Direction3D, Direction3D);
as_vector_add!(Direction3D, Vector3D);
as_vector_add!(Vector3D, Direction3D);
as_vector_sub!(Direction3D, Direction3D);
as_vector_sub!(Direction3D, Vector3D);
as_vector_sub!(Vector3D, Direction3D);
as_vector_mul!(Direction3D);
as_vector_div!(Direction3D);

impl<'a> Neg for &'a Direction3D {
    type Output = Direction3D;
    
    fn neg(self) -> Direction3D {
        Direction3D::from_normalized_vector(&-(&self.direction))
    }
}

impl Neg for Direction3D {
    type Output = Direction3D;
    
    fn neg(self) -> Direction3D {
        -&self
    }
}

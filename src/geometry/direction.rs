use std::num::{Float};

use super::{Point3D, Vector3D, Matrix3D};

#[deriving(PartialEq, PartialOrd, Clone, Show)]
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
    
    pub fn from_vector(vector: &Vector3D) -> Direction3D {
        let magnitude = vector.magnitude();
        if magnitude > 0.0 {
            Direction3D::from_normalized_vector(&vector.scale(1.0 / magnitude))
        } else {
            Direction3D::from_normalized_vector(vector)
        }
    }
    
    fn from_normalized_vector(normalized_vector: &Vector3D) -> Direction3D {
        Direction3D {direction: *normalized_vector}
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
    
    pub fn as_vector(&self) -> &Vector3D {
        &self.direction
    }
    
    pub fn to_vector(&self) -> Vector3D {
        Vector3D {
            x: self.direction.x,
            y: self.direction.y,
            z: self.direction.z
        }
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
}

impl Neg<Direction3D> for Direction3D {
    fn neg(&self) -> Direction3D {
        Direction3D::from_normalized_vector(&self.as_vector().neg())
    }
}

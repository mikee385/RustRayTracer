use std::num::{Float};

use super::{Point3D, Direction3D, Matrix3D};

#[deriving(PartialEq, PartialOrd, Copy, Clone, Show)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

static VECTOR3D_ZERO: Vector3D = Vector3D {
    x: 0.0, 
    y: 0.0, 
    z: 0.0
};

impl Vector3D {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Vector3D {
        Vector3D {
            x: x, 
            y: y, 
            z: z
        }
    }
    
    pub fn between_points(from: &Point3D, to: &Point3D) -> Vector3D {
        Vector3D::from_xyz(
            to.x - from.x, 
            to.y - from.y, 
            to.z - from.z
        )
    }
    
    pub fn from_point(point: &Point3D) -> Vector3D {
        Vector3D::from_xyz(
            point.x, 
            point.y, 
            point.z
        )
    }
    
    pub fn zero() -> &'static Vector3D {
        &VECTOR3D_ZERO
    }
    
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn to_unit(&self) -> Direction3D {
        Direction3D::from_vector(self)
    }
    
    pub fn to_orthonormal_basis(&self) -> Matrix3D {
        Direction3D::from_vector(self).to_orthonormal_basis()
    }
    
    pub fn scale(&self, scale: f32) -> Vector3D {
        Vector3D::from_xyz(
            self.x * scale, 
            self.y * scale, 
            self.z * scale
        )
    }
    
    pub fn projection_dir(&self, direction: &Direction3D) -> Vector3D {
        direction.as_vector() * Vector3D::dot(self, direction.as_vector())
    }
    
    pub fn projection_vec(&self, direction: &Vector3D) -> Vector3D {
        let denominator = Vector3D::dot(direction, direction);
        if denominator > 0.0 {
            direction * (Vector3D::dot(self, direction) / denominator)
        } else {
            *Vector3D::zero()
        }
    }

    pub fn rotate(&self, matrix: &Matrix3D) -> Vector3D
    {
        Vector3D::from_xyz(
            self.x * matrix.x.x + self.y * matrix.y.x + self.z * matrix.z.x,
            self.x * matrix.x.y + self.y * matrix.y.y + self.z * matrix.z.y,
            self.x * matrix.x.z + self.y * matrix.y.z + self.z * matrix.z.z
        )
    }
    
    pub fn eq_tol(&self, other: &Vector3D, tolerance: f32) -> bool {
        (self.x - other.x).abs() < tolerance &&
        (self.y - other.z).abs() < tolerance &&
        (self.y - other.z).abs() < tolerance
    }

    pub fn dot(vector1: &Vector3D, vector2: &Vector3D) -> f32
    {
        vector1.x * vector2.x + 
        vector1.y * vector2.y + 
        vector1.z * vector2.z
    }

    pub fn cross(vector1: &Vector3D, vector2: &Vector3D) -> Vector3D
    {
        Vector3D::from_xyz(
            vector1.y * vector2.z - vector1.z * vector2.y,
            vector1.z * vector2.x - vector1.x * vector2.z,
            vector1.x * vector2.y - vector1.y * vector2.x
        )
    }
}

impl<'a, 'b> Add<&'a Vector3D, Vector3D> for &'b Vector3D {
    fn add(self, other: &Vector3D) -> Vector3D {
        Vector3D::from_xyz(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z
        )
    }
}

impl<'a> Add<&'a Vector3D, Vector3D> for Vector3D {
    fn add(self, other: &Vector3D) -> Vector3D {
        &self + other
    }
}

impl<'a> Add<Vector3D, Vector3D> for &'a Vector3D {
    fn add(self, other: Vector3D) -> Vector3D {
        self + &other
    }
}

impl Add<Vector3D, Vector3D> for Vector3D {
    fn add(self, other: Vector3D) -> Vector3D {
        &self + &other
    }
}

impl<'a, 'b> Sub<&'a Vector3D, Vector3D> for &'b Vector3D {
    fn sub(self, other: &Vector3D) -> Vector3D {
        Vector3D::from_xyz(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z
        )
    }
}

impl<'a> Sub<&'a Vector3D, Vector3D> for Vector3D {
    fn sub(self, other: &Vector3D) -> Vector3D {
        &self - other
    }
}

impl<'a> Sub<Vector3D, Vector3D> for &'a Vector3D {
    fn sub(self, other: Vector3D) -> Vector3D {
        self - &other
    }
}

impl Sub<Vector3D, Vector3D> for Vector3D {
    fn sub(self, other: Vector3D) -> Vector3D {
        &self - &other
    }
}

impl<'a> Mul<f32, Vector3D> for &'a Vector3D {
    fn mul(self, scale: f32) -> Vector3D {
        self.scale(scale)        
    }
}

impl Mul<f32, Vector3D> for Vector3D {
    fn mul(self, scale: f32) -> Vector3D {
        &self * scale
    }
}

impl<'a> Div<f32, Vector3D> for &'a Vector3D {
    fn div(self, scale: f32) -> Vector3D {
        let inv_scale = 1.0 / scale;
        self.scale(inv_scale)        
    }
}

impl Div<f32, Vector3D> for Vector3D {
    fn div(self, scale: f32) -> Vector3D {
        &self / scale       
    }
}

impl Neg<Vector3D> for Vector3D {
    fn neg(&self) -> Vector3D {
        Vector3D::from_xyz(
            -self.x,
            -self.y,
            -self.z
        )
    }
}

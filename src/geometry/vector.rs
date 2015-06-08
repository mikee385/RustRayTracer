use std::ops::{Add, Sub, Mul, Div, Neg};

use super::{Point3D, Direction3D, Matrix3D};

#[derive(PartialEq, PartialOrd, Clone, Debug)]
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
        direction * Vector3D::dot(self, direction)
    }
    
    pub fn projection_vec(&self, direction: &Vector3D) -> Vector3D {
        let denominator = Vector3D::dot(direction, direction);
        if denominator > 0.0 {
            direction * (Vector3D::dot(self, direction) / denominator)
        } else {
            Vector3D::zero().clone()
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
    
    pub fn eq_tol<T: AsVector>(&self, other: &T, tolerance: f32) -> bool {
        let v = other.as_vector();
        (self.x - v.x).abs() < tolerance &&
        (self.y - v.z).abs() < tolerance &&
        (self.y - v.z).abs() < tolerance
    }
}

//------------------------------------------------------------------------------

pub trait AsVector {
    fn as_vector<'a>(&'a self) -> &'a Vector3D;
}

impl Vector3D {
    pub fn dot<T: AsVector, U: AsVector>(vector1: &T, vector2: &U) -> f32
    {
        let v1 = vector1.as_vector();
        let v2 = vector2.as_vector();

        v1.x * v2.x + 
        v1.y * v2.y + 
        v1.z * v2.z
    }

    pub fn cross<T: AsVector, U: AsVector>(vector1: &T, vector2: &U) -> Vector3D
    {
        let v1 = vector1.as_vector();
        let v2 = vector2.as_vector();

        Vector3D::from_xyz(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x
        )
    }
}

#[macro_export]
macro_rules! as_vector_add {
    ($T:ty, $U:ty) => (
        impl<'a, 'b> Add<&'a $U> for &'b $T {
            type Output = Vector3D;
            
            fn add(self, other: &$U) -> Vector3D {
                let v1 = self.as_vector();
                let v2 = other.as_vector();

                Vector3D::from_xyz(
                    v1.x + v2.x,
                    v1.y + v2.y,
                    v1.z + v2.z
                )
            }
        }

        impl<'a> Add<&'a $U> for $T {
            type Output = Vector3D;
            
            fn add(self, other: &$U) -> Vector3D {
                &self + other
            }
        }

        impl<'a> Add<$U> for &'a $T {
            type Output = Vector3D;
            
            fn add(self, other: $U) -> Vector3D {
                self + &other
            }
        }

        impl Add<$U> for $T {
            type Output = Vector3D;
            
            fn add(self, other: $U) -> Vector3D {
                &self + &other
            }
        }
    )
}

#[macro_export]
macro_rules! as_vector_sub {
    ($T:ty, $U:ty) => (
        impl<'a, 'b> Sub<&'a $U> for &'b $T {
            type Output = Vector3D;
            
            fn sub(self, other: &$U) -> Vector3D {
                let v1 = self.as_vector();
                let v2 = other.as_vector();

                Vector3D::from_xyz(
                    v1.x - v2.x,
                    v1.y - v2.y,
                    v1.z - v2.z
                )
            }
        }

        impl<'a> Sub<&'a $U> for $T {
            type Output = Vector3D;
            
            fn sub(self, other: &$U) -> Vector3D {
                &self - other
            }
        }

        impl<'a> Sub<$U> for &'a $T {
            type Output = Vector3D;
            
            fn sub(self, other: $U) -> Vector3D {
                self - &other
            }
        }

        impl Sub<$U> for $T {
            type Output = Vector3D;
            
            fn sub(self, other: $U) -> Vector3D {
                &self - &other
            }
        }
    )
}

#[macro_export]
macro_rules! as_vector_mul {
    ($T:ty) => (
        impl<'a> Mul<f32> for &'a $T {
            type Output = Vector3D;
            
            fn mul(self, scale: f32) -> Vector3D {
                self.as_vector().scale(scale)
            }
        }

        impl Mul<f32> for $T {
            type Output = Vector3D;
            
            fn mul(self, scale: f32) -> Vector3D {
                &self * scale
            }
        }
    )
}

#[macro_export]
macro_rules! as_vector_div {
    ($T:ty) => (
        impl<'a> Div<f32> for &'a $T {
            type Output = Vector3D;
            
            fn div(self, scale: f32) -> Vector3D {
                let inv_scale = 1.0 / scale;
                self.as_vector().scale(inv_scale)        
            }
        }

        impl Div<f32> for $T {
            type Output = Vector3D;
            
            fn div(self, scale: f32) -> Vector3D {
                &self / scale       
            }
        }
    )
}

//------------------------------------------------------------------------------

impl AsVector for Vector3D {
    fn as_vector<'a>(&'a self) -> &'a Vector3D {
        self
    }
}

as_vector_add!(Vector3D, Vector3D);
as_vector_sub!(Vector3D, Vector3D);
as_vector_mul!(Vector3D);
as_vector_div!(Vector3D);

impl<'a> Neg for &'a Vector3D {
    type Output = Vector3D;
    
    fn neg(self) -> Vector3D {
        Vector3D::from_xyz(
            -self.x,
            -self.y,
            -self.z
        )
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;
    
    fn neg(self) -> Vector3D {
        -&self
    }
}

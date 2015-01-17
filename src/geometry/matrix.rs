use super::{Vector3D, AsVector};

#[derive(PartialEq, PartialOrd, Clone, Show)]
pub struct Matrix3D {
    pub x: Vector3D,
    pub y: Vector3D,
    pub z: Vector3D
}

static MATRIX3D_IDENTITY: Matrix3D = Matrix3D {
    x: Vector3D {x: 1.0, y: 0.0, z: 0.0},
    y: Vector3D {x: 0.0, y: 1.0, z: 0.0},
    z: Vector3D {x: 0.0, y: 0.0, z: 1.0}
};

impl Matrix3D {
    pub fn new<T: AsVector, U: AsVector, V: AsVector>(x: &T, y: &U, z: &V) -> Matrix3D {
        Matrix3D {
            x: x.as_vector().clone(),
            y: y.as_vector().clone(),
            z: z.as_vector().clone()
        }
    }
    
    pub fn identity() -> &'static Matrix3D {
        &MATRIX3D_IDENTITY
    }
    
    pub fn to_orthonormal_basis(&self) -> Matrix3D {
        let unit_x = self.x.to_unit();        
        let unit_y = (&self.y - unit_x.as_vector() * Vector3D::dot(&unit_x, &self.y)).to_unit();        
        let unit_z = (&self.z - unit_x.as_vector() * Vector3D::dot(&unit_x, &self.z) - unit_y.as_vector() * Vector3D::dot(&unit_y, &self.z)).to_unit();

        Matrix3D::new(&unit_x, &unit_y, &unit_z)
    }
}

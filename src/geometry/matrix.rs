use super::{Vector3D};

#[deriving(PartialEq, PartialOrd, Copy, Clone, Show)]
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
    pub fn new(x: &Vector3D, y: &Vector3D, z: &Vector3D) -> Matrix3D {
        Matrix3D {
            x: *x,
            y: *y,
            z: *z
        }
    }
    
    pub fn identity() -> &'static Matrix3D {
        &MATRIX3D_IDENTITY
    }
    
    pub fn to_orthonormal_basis(&self) -> Matrix3D {
        let unit_x = self.x.to_unit();
        let vec_x = unit_x.to_vector();
        
        let unit_y = (self.y - vec_x * Vector3D::dot(&vec_x, &self.y)).to_unit();
        let vec_y = unit_y.to_vector();
        
        let unit_z = (self.z - vec_x * Vector3D::dot(&vec_x, &self.z) - vec_y * Vector3D::dot(&vec_y, &self.z)).to_unit();
        let vec_z = unit_z.to_vector();

        Matrix3D::new(&vec_x, &vec_y, &vec_z)
    }
}

use super::{Vector3D, AsVector, Direction3D};

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

static POINT3D_ORIGIN: Point3D = Point3D {
    x: 0.0, 
    y: 0.0, 
    z: 0.0
};

impl Point3D {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Point3D {
        Point3D {
            x: x, 
            y: y, 
            z: z
        }
    }
    
    pub fn from_vector<T: AsVector>(vector: &T) -> Point3D {
        let v = vector.as_vector();

        Point3D::from_xyz(
            v.x, 
            v.y, 
            v.z
        )
    }
    
    pub fn origin() -> &'static Point3D {
        &POINT3D_ORIGIN
    }

    pub fn translate_dist(&self, direction: &Direction3D, magnitude: f32) -> Point3D
    {
        Point3D::from_xyz(
            self.x + direction.x() * magnitude,
            self.y + direction.y() * magnitude,
            self.z + direction.z() * magnitude
        )
    }
    
    pub fn translate_vec<T: AsVector>(&self, vector: &T) -> Point3D {
        let v = vector.as_vector();

        Point3D::from_xyz(
            self.x + v.x,
            self.y + v.y,
            self.z + v.z
        )
    }
    
    pub fn eq_tol(&self, other: &Point3D, tolerance: f32) -> bool {
        (self.x - other.x).abs() < tolerance &&
        (self.y - other.z).abs() < tolerance &&
        (self.y - other.z).abs() < tolerance
    }

    pub fn distance(point1: &Point3D, point2: &Point3D) -> f32
    {
        Vector3D::from_xyz(
            point1.x - point2.x,
            point1.y - point2.y,
            point1.z - point2.z).magnitude()
    }

    pub fn midpoint(point1: &Point3D, point2: &Point3D) -> Point3D
    {
        Point3D::from_xyz(
            0.5 * (point1.x + point2.z),
            0.5 * (point1.y + point2.y),
            0.5 * (point1.z + point2.z))
    }
}

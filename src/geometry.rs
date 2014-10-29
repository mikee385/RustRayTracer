#![allow(dead_code)]

use std::f32::consts::PI;

pub const EPSILON: f32 = 1.0E-9;
pub const DEGREES_TO_RADIANS: f32 = PI / 180.0;

static POINT3D_ORIGIN: Point3D = Point3D {
    x: 0.0, 
    y: 0.0, 
    z: 0.0
};

pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point3D {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Point3D {
        Point3D {
            x: x, 
            y: y, 
            z: z
        }
    }
    
    pub fn from_vector(vector: &Vector3D) -> Point3D {
        Point3D::from_xyz(
            vector.x, 
            vector.y, 
            vector.z
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
    
    pub fn translate_vec(&self, vector: &Vector3D) -> Point3D {
        Point3D::from_xyz(
            self.x + vector.x,
            self.y + vector.y,
            self.z + vector.z
        )
    }
    
    pub fn rotate(&self, matrix: &Matrix3D) -> Point3D {
        Point3D::from_xyz(
            self.x * matrix.x.x + self.y * matrix.y.x + self.z * matrix.z.x,
            self.x * matrix.x.y + self.y * matrix.y.y + self.z * matrix.z.y,
            self.x * matrix.x.z + self.y * matrix.y.z + self.z * matrix.z.z)
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

impl Clone for Point3D {
    fn clone(&self) -> Point3D {
        Point3D {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
    
    fn clone_from(&mut self, source: &Point3D) {
        self.x = source.x;
        self.y = source.y;
        self.z = source.z;
    }
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Point3D) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

impl PartialOrd for Point3D {
    fn partial_cmp(&self, other: &Point3D) -> Option<Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(Less) => Some(Less),
            Some(Greater) => Some(Greater),
            Some(Equal) => match self.y.partial_cmp(&other.y) {
                Some(Less) => Some(Less),
                Some(Greater) => Some(Greater),
                Some(Equal) => match self.z.partial_cmp(&other.z) {
                    Some(Less) => Some(Less),
                    Some(Greater) => Some(Greater),
                    Some(Equal) => Some(Equal),
                    None => None
                },
                None => None
            },
            None => None
        }
    }
}

static VECTOR3D_ZERO: Vector3D = Vector3D {
    x: 0.0, 
    y: 0.0, 
    z: 0.0
};

pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

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
            *direction * (Vector3D::dot(self, direction) / denominator)
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

impl Clone for Vector3D {
    fn clone(&self) -> Vector3D {
        Vector3D {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
    
    fn clone_from(&mut self, source: &Vector3D) {
        self.x = source.x;
        self.y = source.y;
        self.z = source.z;
    }
}

impl Add<Vector3D, Vector3D> for Vector3D {
    fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D::from_xyz(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z
        )
    }
}

impl Sub<Vector3D, Vector3D> for Vector3D {
    fn sub(&self, other: &Vector3D) -> Vector3D {
        Vector3D::from_xyz(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z
        )
    }
}

impl Mul<f32, Vector3D> for Vector3D {
    fn mul(&self, scale: &f32) -> Vector3D {
        self.scale(*scale)        
    }
}

impl Div<f32, Vector3D> for Vector3D {
    fn div(&self, scale: &f32) -> Vector3D {
        let inv_scale = 1.0 / *scale;
        self.scale(inv_scale)        
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

impl PartialEq for Vector3D {
    fn eq(&self, other: &Vector3D) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

impl PartialOrd for Vector3D {
    fn partial_cmp(&self, other: &Vector3D) -> Option<Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(Less) => Some(Less),
            Some(Greater) => Some(Greater),
            Some(Equal) => match self.y.partial_cmp(&other.y) {
                Some(Less) => Some(Less),
                Some(Greater) => Some(Greater),
                Some(Equal) => match self.z.partial_cmp(&other.z) {
                    Some(Less) => Some(Less),
                    Some(Greater) => Some(Greater),
                    Some(Equal) => Some(Equal),
                    None => None
                },
                None => None
            },
            None => None
        }
    }
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

pub struct Direction3D {
    direction: Vector3D
}

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

impl Clone for Direction3D {
    fn clone(&self) -> Direction3D {
        Direction3D {
            direction: self.direction
        }
    }
    
    fn clone_from(&mut self, source: &Direction3D) {
        self.direction = source.direction;
    }
}

impl Neg<Direction3D> for Direction3D {
    fn neg(&self) -> Direction3D {
        Direction3D::from_normalized_vector(&self.as_vector().neg())
    }
}

impl PartialEq for Direction3D {
    fn eq(&self, other: &Direction3D) -> bool {
        self.as_vector().eq(other.as_vector())
    }
}

impl PartialOrd for Direction3D {
    fn partial_cmp(&self, other: &Direction3D) -> Option<Ordering> {
        match self.x().partial_cmp(&other.x()) {
            Some(Less) => Some(Less),
            Some(Greater) => Some(Greater),
            Some(Equal) => match self.y().partial_cmp(&other.y()) {
                Some(Less) => Some(Less),
                Some(Greater) => Some(Greater),
                Some(Equal) => match self.z().partial_cmp(&other.z()) {
                    Some(Less) => Some(Less),
                    Some(Greater) => Some(Greater),
                    Some(Equal) => Some(Equal),
                    None => None
                },
                None => None
            },
            None => None
        }
    }
}

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

impl Clone for Ray3D {
    fn clone(&self) -> Ray3D {
        Ray3D {
            origin: self.origin,
            direction: self.direction
        }
    }
    
    fn clone_from(&mut self, source: &Ray3D) {
        self.origin = source.origin;
        self.direction = source.direction;
    }
}

impl PartialEq for Ray3D {
    fn eq(&self, other: &Ray3D) -> bool {
        self.origin.eq(&other.origin) &&
        self.direction.eq(&other.direction)
    }
}

impl PartialOrd for Ray3D {
    fn partial_cmp(&self, other: &Ray3D) -> Option<Ordering> {
        match self.origin.partial_cmp(&other.origin) {
            Some(Less) => Some(Less),
            Some(Greater) => Some(Greater),
            Some(Equal) => match self.direction.partial_cmp(&other.direction) {
                Some(Less) => Some(Less),
                Some(Greater) => Some(Greater),
                Some(Equal) => Some(Equal),
                None => None
            },
            None => None
        }
    }
}

static MATRIX3D_IDENTITY: Matrix3D = Matrix3D {
    x: Vector3D {x: 1.0, y: 0.0, z: 0.0},
    y: Vector3D {x: 0.0, y: 1.0, z: 0.0},
    z: Vector3D {x: 0.0, y: 0.0, z: 1.0}
};

pub struct Matrix3D {
    pub x: Vector3D,
    pub y: Vector3D,
    pub z: Vector3D
}

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
        let vec_x = unit_x.as_vector();
        
        let unit_y = (self.y - vec_x * Vector3D::dot(vec_x, &self.y)).to_unit();
        let vec_y = unit_y.as_vector();
        
        let unit_z = (self.z - vec_x * Vector3D::dot(vec_x, &self.z) - vec_y * Vector3D::dot(vec_y, &self.z)).to_unit();
        let vec_z = unit_z.as_vector();

        Matrix3D::new(vec_x, vec_y, vec_z)
    }
}

impl Clone for Matrix3D {
    fn clone(&self) -> Matrix3D {
        Matrix3D {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
    
    fn clone_from(&mut self, source: &Matrix3D) {
        self.x = source.x;
        self.y = source.y;
        self.z = source.z;
    }
}

impl PartialEq for Matrix3D {
    fn eq(&self, other: &Matrix3D) -> bool {
        self.x.eq(&other.x) &&
        self.y.eq(&other.y) &&
        self.z.eq(&other.z)
    }
}

impl PartialOrd for Matrix3D {
    fn partial_cmp(&self, other: &Matrix3D) -> Option<Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(Less) => Some(Less),
            Some(Greater) => Some(Greater),
            Some(Equal) => match self.y.partial_cmp(&other.y) {
                Some(Less) => Some(Less),
                Some(Greater) => Some(Greater),
                Some(Equal) => match self.z.partial_cmp(&other.z) {
                    Some(Less) => Some(Less),
                    Some(Greater) => Some(Greater),
                    Some(Equal) => Some(Equal),
                    None => None
                },
                None => None
            },
            None => None
        }
    }
}
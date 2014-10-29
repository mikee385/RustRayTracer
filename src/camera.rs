use geometry::{DEGREES_TO_RADIANS, Point3D, Vector3D, Direction3D, Ray3D, Matrix3D};
use color::{ColorRGB};
use table::{Table};

const DEFAULT_COLOR: ColorRGB = ColorRGB {red: 0.0, green: 0.0, blue: 0.0};

pub struct Camera {
    image: Table<ColorRGB>,     
    position: Point3D,
    orientation: Matrix3D,
    
    x_min: f32,
    y_max: f32,
    dx: f32,
    dy: f32,    
    distance_to_plane: f32
}

impl Camera {
    pub fn from_fov(image_width: uint, image_height: uint, field_of_view: f32, distance_to_plane: f32, position: &Point3D, look_at_point: &Point3D) -> Camera {
        let y_max = (field_of_view / 2.0 * DEGREES_TO_RADIANS).tan() * distance_to_plane;
        let x_min = -y_max * (image_width as f32) / (image_height as f32);    
        Camera {
            image: Table::from_elem(image_width, image_height, DEFAULT_COLOR),
            position: *position,
            orientation: compute_orientation(position, look_at_point),
            x_min: x_min,
            y_max: y_max,
            dx: -2.0 * x_min / (image_width as f32),
            dy: 2.0 * y_max / (image_height as f32),
            distance_to_plane: distance_to_plane
        }
    }
    
    pub fn from_dimensions(image_width: uint, image_height: uint, plane_width: f32, plane_height: f32, distance_to_plane: f32, position: &Point3D, look_at_point: &Point3D) -> Camera {
        Camera {
            image: Table::from_elem(image_width, image_height, DEFAULT_COLOR),
            position: *position,
            orientation: compute_orientation(position, look_at_point),
            x_min: -plane_width / 2.0,
            y_max: plane_height / 2.0,
            dx: plane_width / (image_width as f32),
            dy: plane_height / (image_height as f32),
            distance_to_plane: distance_to_plane
        }
    }

    pub fn get_image(&self) -> &Table<ColorRGB> {
        &self.image
    }

    pub fn get_position(&self) -> &Point3D {
        &self.position
    }

    pub fn get_orientation(&self) -> &Matrix3D {
        &self.orientation
    }

    pub fn set_position(&mut self, position: &Point3D) {
        self.position = *position
    }

    pub fn set_orientation(&mut self, orientation: &Matrix3D) {
        self.orientation = *orientation
    }
    
    pub fn get_primary_ray(&self, row: uint, column: uint) -> Ray3D {
        let point_in_camera = self.get_pixel_center(row, column);
        let ray_direction = Direction3D::between_points(&self.position, &self.convert_camera_to_world(&point_in_camera));
        Ray3D::new(&self.position, &ray_direction)
    }
    
    pub fn get_sub_rays(&self, row: uint, column: uint, rays: &mut Table<Ray3D>) {
        let width = rays.get_width();
        if width < 2 {
            fail!("Camera::get_sub_rays: `width` of `rays` table is too small ({} < {})", width, 2u)
        }
        
        let height = rays.get_height();
        if height < 2 {
            fail!("Camera::get_sub_rays: `height` of `rays` table is too small ({} < {})", height, 2u)
        }
    
        let x_step = self.dx / ((width - 1) as f32);
        let y_step = self.dy / ((height - 1) as f32);

        let x0 = self.x_min + self.dx * (column as f32);
        let y0 = self.y_max - self.dy * (row as f32);
        let z0 = self.distance_to_plane;

        for row in range(0, height) {
            for column in range(0, width) {
                let point_in_camera = Point3D::from_xyz(x0 + (column as f32)*x_step, y0 - (row as f32)*y_step, z0);
                let ray_direction = Direction3D::between_points(&self.position, &self.convert_camera_to_world(&point_in_camera));
                rays.set(row, column, Ray3D::new(&self.position, &ray_direction));
            }
        }
    }
    
    pub fn get_pixel(&self, row: uint, column: uint) -> &ColorRGB {
        self.image.get(row, column)
    }
    
    pub fn set_pixel(&mut self, row: uint, column: uint, pixel: ColorRGB) {
        self.image.set(row, column, pixel)
    }
    
    fn get_pixel_center(&self, row: uint, column: uint) -> Point3D {
        let x = self.x_min + self.dx * ((column as f32) + 0.5);
        let y = self.y_max - self.dy * ((row as f32) + 0.5);
        let z = self.distance_to_plane;

        Point3D::from_xyz(x, y, z)
    }
    
    fn convert_camera_to_world(&self, point_in_camera: &Point3D) -> Point3D {
        point_in_camera.rotate(&self.orientation).translate_vec(&Vector3D::from_point(&self.position))
    }
}

fn compute_orientation(position: &Point3D, look_at_point: &Point3D) -> Matrix3D {
    let z = Direction3D::between_points(position, look_at_point).to_vector();
    let x = Vector3D::cross(Direction3D::unit_y().as_vector(), &z).to_unit().to_vector();
    let y = Vector3D::cross(&z, &x).to_unit().to_vector();
    
    Matrix3D::new(&x, &y, &z)
}
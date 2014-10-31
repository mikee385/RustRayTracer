#![allow(dead_code)]
#![allow(unused_variables)]

use geometry::{DEGREES_TO_RADIANS, Point3D, Vector3D, Direction3D, Ray3D, Matrix3D};
use color::{ColorRGB};
use table::{Table};

pub struct Camera {   
    position: Point3D,
    orientation: Matrix3D,
    
    x_min: f32,
    y_max: f32,
    dx: f32,
    dy: f32,    
    distance_to_plane: f32
}

impl Camera {
    pub fn from_fov(pixel_table: &Table<ColorRGB>, field_of_view: f32, distance_to_plane: f32, position: &Point3D, look_at_point: &Point3D) -> Camera {
        let table_width = pixel_table.get_width() as f32;
        let table_height = pixel_table.get_height() as f32;

        let y_max = (field_of_view / 2.0 * DEGREES_TO_RADIANS).tan() * distance_to_plane;
        let x_min = -y_max * (table_width as f32) / (table_height as f32);

        Camera {
            position: *position,
            orientation: compute_orientation(position, look_at_point),
            x_min: x_min,
            y_max: y_max,
            dx: -2.0 * x_min / table_width,
            dy: 2.0 * y_max / table_height,
            distance_to_plane: distance_to_plane
        }
    }
    
    pub fn from_dimensions(pixel_table: &Table<ColorRGB>, plane_width: f32, plane_height: f32, distance_to_plane: f32, position: &Point3D, look_at_point: &Point3D) -> Camera {
        let table_width = pixel_table.get_width() as f32;
        let table_height = pixel_table.get_height() as f32;

        Camera {
            position: *position,
            orientation: compute_orientation(position, look_at_point),
            x_min: -plane_width / 2.0,
            y_max: plane_height / 2.0,
            dx: plane_width / table_width,
            dy: plane_height / table_height,
            distance_to_plane: distance_to_plane
        }
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
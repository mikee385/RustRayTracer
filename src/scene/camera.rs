use std::num::{Float};

use geometry::{DEGREES_TO_RADIANS, Point3D, Vector3D, Direction3D, Ray3D, Matrix3D};
use table::{Table};

#[derive(Show)]
pub struct Camera {   
    position: Point3D,
    orientation: Matrix3D,
    image_dimensions: (usize, usize),
    
    x_min: f32,
    y_max: f32,
    dx: f32,
    dy: f32,    
    distance_to_plane: f32
}

impl Camera {
    pub fn from_fov(image_dimensions: (usize, usize), field_of_view: f32, distance_to_plane: f32, position: &Point3D, look_at_point: &Point3D) -> Camera {
        let (image_width_usize, image_height_usize) = image_dimensions;
        let image_width = image_width_usize as f32;
        let image_height = image_height_usize as f32;

        let y_max = (field_of_view / 2.0 * DEGREES_TO_RADIANS).tan() * distance_to_plane;
        let x_min = -y_max * image_width / image_height;

        Camera {
            position: position.clone(),
            orientation: compute_orientation(position, look_at_point),
            image_dimensions: image_dimensions,
            x_min: x_min,
            y_max: y_max,
            dx: -2.0 * x_min / image_width,
            dy: 2.0 * y_max / image_height,
            distance_to_plane: distance_to_plane
        }
    }
    
    pub fn from_dimensions(image_dimensions: (usize, usize), plane_dimensions: (f32, f32), distance_to_plane: f32, position: &Point3D, look_at_point: &Point3D) -> Camera {
        let (image_width_usize, image_height_usize) = image_dimensions;
        let image_width = image_width_usize as f32;
        let image_height = image_height_usize as f32;

        let (plane_width, plane_height) = plane_dimensions;

        Camera {
            position: position.clone(),
            orientation: compute_orientation(position, look_at_point),
            image_dimensions: image_dimensions,
            x_min: -plane_width / 2.0,
            y_max: plane_height / 2.0,
            dx: plane_width / image_width,
            dy: plane_height / image_height,
            distance_to_plane: distance_to_plane
        }
    }

    pub fn get_position(&self) -> &Point3D {
        &self.position
    }

    pub fn get_orientation(&self) -> &Matrix3D {
        &self.orientation
    }

    pub fn get_image_dimensions(&self) -> (usize, usize) {
        self.image_dimensions
    }
    
    pub fn get_primary_ray(&self, index: (usize, usize)) -> Ray3D {
        let point_in_camera = self.get_pixel_center(index);
        let ray_direction = Direction3D::between_points(&self.position, &self.convert_camera_to_world(&point_in_camera));
        Ray3D::new(&self.position, &ray_direction)
    }
    
    pub fn get_sub_rays(&self, index: (usize, usize), rays: &mut Table<Ray3D>) {
        let (width, height) = rays.get_dimensions();
        if width < 2 {
            panic!("Camera::get_sub_rays: `width` of `rays` table is too small ({} < {})", width, 2)
        }
        if height < 2 {
            panic!("Camera::get_sub_rays: `height` of `rays` table is too small ({} < {})", height, 2)
        }
        let (row, column) = index;
    
        let x_step = self.dx / ((width - 1) as f32);
        let y_step = self.dy / ((height - 1) as f32);

        let x0 = self.x_min + self.dx * (column as f32);
        let y0 = self.y_max - self.dy * (row as f32);
        let z0 = self.distance_to_plane;

        for ((row, column), value) in rays.iter_mut().enumerate_2d() {
                let point_in_camera = Point3D::from_xyz(x0 + (column as f32)*x_step, y0 - (row as f32)*y_step, z0);
                let ray_direction = Direction3D::between_points(&self.position, &self.convert_camera_to_world(&point_in_camera));
                *value = Ray3D::new(&self.position, &ray_direction);
        }
    }
    
    fn get_pixel_center(&self, index: (usize, usize)) -> Point3D {
        let (row, column) = index;
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
    let z = Direction3D::between_points(position, look_at_point);
    let x = Vector3D::cross(Direction3D::unit_y(), &z).to_unit();
    let y = Vector3D::cross(&z, &x).to_unit();
    
    Matrix3D::new(&x, &y, &z)
}

use std::vec::{Vec};

use geometry::{Point3D, Vector3D, Direction3D, Ray3D};
use color::{ColorRGB};
use scene_object::{SceneObject};
use scene_light::{SceneLight};
use table::{Table};
use camera::{Camera};

const BIAS: f32 = 1.0e-4;

pub struct Scene<'a> {
    background_color: ColorRGB,
    refractive_index: f32,
    max_ray_depth: u32,
    
    items: Vec<InternalObject<'a>>,
    lights: Vec<InternalLight<'a>>
}

impl<'a> Scene<'a> {
    pub fn new(background_color: &ColorRGB, refractive_index: f32, max_ray_depth: u32) -> Scene {
        Scene {
            background_color: *background_color,
            refractive_index: refractive_index,
            max_ray_depth: max_ray_depth,
            items: Vec::new(),
            lights: Vec::new()
        }
    }
    
    pub fn add_light_source(&mut self, light: &'a SceneLight) {
        let index = self.items.len();
        self.items.push(InternalObject {
            index: index,
            object: light.as_scene_object(), 
            is_light: true
        });
        self.lights.push(InternalLight {
            index: index,
            light: light
        });
    }
    
    pub fn add_object(&mut self, object: &'a SceneObject) {
        let index = self.items.len();
        self.items.push(InternalObject {
            index: index,
            object: object, 
            is_light: false
        });
    }
    
    pub fn render(&self, camera: &mut Camera) {
        let width = camera.get_image().get_width();
        let height = camera.get_image().get_height();
        
        // Initial Pixel Coloring
        for row in range(0, height) {
            for column in range (0, width) {
                let ray = camera.get_primary_ray(row, column);
                let result = self.trace(&ray, 0);
                
                let result_color = ColorRGB::from_rgb(
                    result.color.red.min(1.0),
                    result.color.green.min(1.0),
                    result.color.blue.min(1.0)
                );
                camera.set_pixel(row, column, result_color);
            }
        }
        
        // Edge Detection
        let mut is_edge = Table::from_elem(width, height, false);
        for row in range(1, height-1) {
            for column in range (1, width-1) {
                let p1 = camera.get_pixel(row - 1, column - 1);
                let p2 = camera.get_pixel(row - 1, column);
                let p3 = camera.get_pixel(row - 1, column + 1);
                let p4 = camera.get_pixel(row, column - 1);
                let p6 = camera.get_pixel(row, column + 1);
                let p7 = camera.get_pixel(row + 1, column - 1);
                let p8 = camera.get_pixel(row + 1, column);
                let p9 = camera.get_pixel(row + 1, column + 1);

                let r = calculate_gradient(p1.red, p2.red, p3.red, p4.red, p6.red, p7.red, p8.red, p9.red);
                let g = calculate_gradient(p1.green, p2.green, p3.green, p4.green, p6.green, p7.green, p8.green, p9.green);
                let b = calculate_gradient(p1.blue, p2.blue, p3.blue, p4.blue, p6.blue, p7.blue, p8.blue, p9.blue);

                if (r + b + g) > 0.5 {
                    is_edge.set(row, column, true);
                } else {
                    is_edge.set(row, column, false);
                }
            }
        }

        // Anti-aliasing
        let sub_width = 3;
        let sub_height = 3;
        let sub_size = (sub_width * sub_height) as f32;
        let mut sub_rays = Table::from_elem(sub_width, sub_height, Ray3D::new(Point3D::origin(), Direction3D::unit_x()));
        for row in range(0, height) {
            for column in range (0, width) {
                if *is_edge.get(row, column) {
                    let mut pixel_color = *ColorRGB::black();
                    
                    camera.get_sub_rays(row, column, &mut sub_rays);
                    for sub_row in range(0, sub_height) {
                        for sub_column in range(0, sub_width) {
                            let result = self.trace(sub_rays.get(sub_row, sub_column), 0);
                            
                            pixel_color = ColorRGB::from_rgb(
                                pixel_color.red + result.color.red / sub_size,
                                pixel_color.green + result.color.green / sub_size,
                                pixel_color.blue + result.color.blue / sub_size
                            );
                        }
                    }
                    camera.set_pixel(row, column, pixel_color);
                }
            }
        }
    }

    fn trace(&self, ray: &Ray3D, depth: u32) -> TraceResult {

        // Find the nearest object that the ray intersects.
        let mut optional_nearest: Option<(&InternalObject, f32)> = None;
        for current_item in self.items.iter() {
            let optional_intersection = current_item.object.intersect(ray);
            if optional_intersection.is_some() {
                let current_distance = optional_intersection.unwrap();
                match optional_nearest {
                    Some((_, nearest_distance)) => {
                        if current_distance < nearest_distance {
                            optional_nearest = Some((current_item, current_distance));
                        }
                    },
                    None => {
                        optional_nearest = Some((current_item, current_distance));
                    }
                }
            }
        }

        // If the ray doesn't hit any objects, return the background color.
        if optional_nearest.is_none() {
            return TraceResult {
                color: self.background_color, 
                distance: 0.0
            };
        }
        
        // Get the information about the nearest intersection.
        let (nearest_item, nearest_distance) = optional_nearest.unwrap();

        // Get the point where the ray intersects the object.
        let point = ray.point(nearest_distance);

        // If the ray intersects a light source, simply return the color of the light.
        if nearest_item.is_light {
            return TraceResult {
                color: nearest_item.object.get_material(&point).color, 
                distance: nearest_distance
            };
        }

        // Get the surface normal and color at the intersection point.
        let normal = nearest_item.object.get_normal(&point);
        let surface_material = nearest_item.object.get_material(&point);
        
        let ray_vector = ray.direction.as_vector();
        let normal_vector = normal.as_vector();
        
        // Calculate the color at the intersection point.
        let mut total_ray_color = *ColorRGB::black();
        
        if depth < self.max_ray_depth {
            // TODO: Add Fresnel effects (?)

            // Calculate the color from the reflected ray.
            let reflection = surface_material.reflection;
            if reflection > 0.0 {
                let reflected_direction = (ray_vector - normal_vector * 2.0 * Vector3D::dot(ray_vector, normal_vector)).to_unit();
                let nearby_point = point.translate_dist(&reflected_direction, BIAS);
                let reflected_result = self.trace(&Ray3D::new(&nearby_point, &reflected_direction), depth + 1);
                total_ray_color = total_ray_color + reflected_result.color * reflection * surface_material.color;
            }

            // Calculate the color from the refracted ray.
            let refraction = surface_material.refraction;
            if refraction > 0.0 {
                let n;
                let cos_i;
                if Vector3D::dot(ray_vector, normal_vector) > 0.0 {
                    // Internal refraction
                    n = surface_material.refractive_index / self.refractive_index;
                    cos_i = -Vector3D::dot(ray_vector, (-normal).as_vector());
                } else {
                    // External refraction
                    n = self.refractive_index / surface_material.refractive_index;
                    cos_i = -Vector3D::dot(ray_vector, normal_vector);
                }

                let cos2_t = 1.0 - n * n * (1.0 - cos_i * cos_i);
                if cos2_t > 0.0 {
                    let refracted_direction = (ray_vector * n + normal_vector * (n * cos_i - (cos2_t).sqrt())).to_unit();
                    let nearby_point = point.translate_dist(&refracted_direction, BIAS);
                    let refracted_result = self.trace(&Ray3D::new(&nearby_point, &refracted_direction), depth + 1);

                    // Beer's Law
                    let absorbance = surface_material.color * (0.15 * -refracted_result.distance);
                    let transparency = ColorRGB::from_rgb(absorbance.red.exp(), absorbance.green.exp(), absorbance.blue.exp());
                    total_ray_color = total_ray_color + refracted_result.color * transparency;
                }
            }
        }
        
        // Calculate the color from each light in the scene.
        for light_item in self.lights.iter() {
            let light = light_item.light;
            let light_color = light.get_material(&point).color;
            let vector_to_light = Vector3D::between_points(&point, light.get_center());
            let distance_to_light = vector_to_light.magnitude();
            let direction_to_light = vector_to_light.to_unit();
            let direction_to_light_vector = direction_to_light.as_vector();

            // Calculate the shading from the light.
            let mut shade: f32 = 1.0;
            let nearby_point = point.translate_dist(&direction_to_light, BIAS);
            let shadow_ray = Ray3D::new(&nearby_point, &direction_to_light);
            for shadow_item in self.items.iter() {
                if shadow_item.index != light_item.index {
                    let shadow_result = shadow_item.object.intersect(&shadow_ray);
                    if shadow_result.is_some() {
                        let distance = shadow_result.unwrap();
                        if distance < distance_to_light {
                            shade = 0.0;
                            break;
                        }
                    }
                }
            }

            if shade != 0.0 {
                // Calculate the diffusive lighting from the light.
                let diffuse = surface_material.diffuse;
                if diffuse > 0.0 {
                    let percentage_of_light = Vector3D::dot(normal_vector, direction_to_light_vector);
                    if percentage_of_light > 0.0 {
                        total_ray_color = total_ray_color + (light_color * surface_material.color) * (shade * diffuse * percentage_of_light);
                    }
                }

                // Calculate the specular lighting from the light.
                let specular = surface_material.specular;
                let shininess = surface_material.shininess;
                if specular > 0.0 && shininess > 0 {
                    let reflected_direction = (direction_to_light_vector - normal_vector * 2.0 * Vector3D::dot(direction_to_light_vector, normal_vector)).to_unit();
                    let percentage_of_light = Vector3D::dot(ray_vector, reflected_direction.as_vector());
                    if percentage_of_light > 0.0 {
                        total_ray_color = total_ray_color + light_color * (shade * specular * percentage_of_light.powi(shininess as i32));
                    }
                }
            }
        }
        
        TraceResult {
            color: total_ray_color,
            distance: nearest_distance
        }
    }
}

struct InternalObject<'a> {
    pub index: uint,
    pub object: &'a SceneObject + 'a,
    pub is_light: bool
}

struct InternalLight<'a> {
    pub index: uint,
    pub light: &'a SceneLight
}

struct TraceResult {
    pub color: ColorRGB,
    pub distance: f32
}

fn calculate_gradient(p1: f32, p2: f32, p3: f32, p4: f32, p6: f32, p7: f32, p8: f32, p9: f32) -> f32
{
    let gx = (p3 + 2.0 * p6 + p9) - (p1 + 2.0 * p4 + p7);
    let gy = (p1 + 2.0 * p2 + p3) - (p7 + 2.0 * p8 + p9);
    (gx*gx + gy*gy).sqrt()
}
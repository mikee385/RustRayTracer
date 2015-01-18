use std::num::{Float};
use std::ops::{Neg};
use std::vec::{Vec};

use geometry::{Vector3D, AsVector, Direction3D, Ray3D};
use color::{ColorRGB};

pub use self::scene_object::{SceneObject};
pub use self::scene_light::{SceneLight};
pub use self::plane::{Plane};
pub use self::sphere::{Sphere};
pub use self::camera::{Camera};

mod scene_object;
mod scene_light;
mod plane;
mod sphere;
mod camera;

const BIAS: f32 = 1.0e-4;

pub struct Scene {
    background_color: ColorRGB,
    refractive_index: f32,
    max_ray_depth: u32,
    
    items: Vec<InternalObject>,
    lights: Vec<InternalLight>
}

impl Scene {
    pub fn new(background_color: &ColorRGB, refractive_index: f32, max_ray_depth: u32) -> Scene {
        Scene {
            background_color: background_color.clone(),
            refractive_index: refractive_index,
            max_ray_depth: max_ray_depth,
            items: Vec::new(),
            lights: Vec::new()
        }
    }
    
    pub fn add_light_source(&mut self, light: Box<SceneLight>) {
        let index = self.items.len();
        self.items.push(InternalObject {
            index: index,
            object: light.clone(), 
            is_light: true
        });
        self.lights.push(InternalLight {
            index: index,
            light: light
        });
    }
    
    pub fn add_object(&mut self, object: Box<SceneObject+Sync+Send>) {
        let index = self.items.len();
        self.items.push(InternalObject {
            index: index,
            object: object, 
            is_light: false
        });
    }

    pub fn trace(&self, ray: &Ray3D, depth: u32) -> TraceResult {

        // Find the nearest object that the ray intersects.
        let mut optional_nearest: Option<(&InternalObject, f32)> = None;
        for current_item in self.items.iter() {
            let optional_intersection = current_item.object.intersect(ray);
            if let Some(current_distance) = optional_intersection {
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
                color: self.background_color.clone(), 
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
        
        // Calculate the color at the intersection point.
        let mut total_ray_color = ColorRGB::black().clone();
        
        if depth < self.max_ray_depth {
            // TODO: Add Fresnel effects (?)

            // Calculate the color from the reflected ray.
            let reflection = surface_material.reflection;
            if reflection > 0.0 {
                let reflected_direction = (&ray.direction - &normal * 2.0 * Vector3D::dot(&ray.direction, &normal)).to_unit();
                let nearby_point = point.translate_dist(&reflected_direction, BIAS);
                let reflected_result = self.trace(&Ray3D::new(&nearby_point, &reflected_direction), depth + 1);
                total_ray_color = &total_ray_color + &reflected_result.color * reflection * &surface_material.color;
            }

            // Calculate the color from the refracted ray.
            let refraction = surface_material.refraction;
            if refraction > 0.0 {
                let n;
                let cos_i;
                if Vector3D::dot(&ray.direction, &normal) > 0.0 {
                    // Internal refraction
                    n = surface_material.refractive_index / self.refractive_index;
                    cos_i = -Vector3D::dot(&ray.direction, &(-&normal));
                } else {
                    // External refraction
                    n = self.refractive_index / surface_material.refractive_index;
                    cos_i = -Vector3D::dot(&ray.direction, &normal);
                }

                let cos2_t = 1.0 - n * n * (1.0 - cos_i * cos_i);
                if cos2_t > 0.0 {
                    let refracted_direction = (&ray.direction * n + &normal * (n * cos_i - (cos2_t).sqrt())).to_unit();
                    let nearby_point = point.translate_dist(&refracted_direction, BIAS);
                    let refracted_result = self.trace(&Ray3D::new(&nearby_point, &refracted_direction), depth + 1);

                    // Beer's Law
                    let absorbance = &surface_material.color * (0.15 * -refracted_result.distance);
                    let transparency = ColorRGB::from_rgb(absorbance.red.exp(), absorbance.green.exp(), absorbance.blue.exp());
                    total_ray_color = &total_ray_color + &refracted_result.color * transparency;
                }
            }
        }
        
        // Calculate the color from each light in the scene.
        for light_item in self.lights.iter() {
            let light = &light_item.light;
            let light_color = light.get_material(&point).color;
            let vector_to_light = Vector3D::between_points(&point, light.get_center());
            let distance_to_light = vector_to_light.magnitude();
            let direction_to_light = vector_to_light.to_unit();

            // Calculate the shading from the light.
            let mut shade: f32 = 1.0;
            let nearby_point = point.translate_dist(&direction_to_light, BIAS);
            let shadow_ray = Ray3D::new(&nearby_point, &direction_to_light);
            for shadow_item in self.items.iter() {
                if shadow_item.index != light_item.index {
                    let shadow_result = shadow_item.object.intersect(&shadow_ray);
                    if let Some(distance) = shadow_result {
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
                    let percentage_of_light = Vector3D::dot(&normal, &direction_to_light);
                    if percentage_of_light > 0.0 {
                        total_ray_color = &total_ray_color + (&light_color * &surface_material.color) * (shade * diffuse * percentage_of_light);
                    }
                }

                // Calculate the specular lighting from the light.
                let specular = surface_material.specular;
                let shininess = surface_material.shininess;
                if specular > 0.0 && shininess > 0 {
                    let reflected_direction = (&direction_to_light - &normal * 2.0 * Vector3D::dot(&direction_to_light, &normal)).to_unit();
                    let percentage_of_light = Vector3D::dot(&ray.direction, &reflected_direction);
                    if percentage_of_light > 0.0 {
                        total_ray_color = &total_ray_color + &light_color * (shade * specular * percentage_of_light.powi(shininess as i32));
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

struct InternalObject {
    pub index: usize,
    pub object: Box<SceneObject+Sync+Send>,
    pub is_light: bool
}

struct InternalLight {
    pub index: usize,
    pub light: Box<SceneLight>
}

#[derive(Show)]
pub struct TraceResult {
    pub color: ColorRGB,
    pub distance: f32
}

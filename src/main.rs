#![allow(dead_code)]

extern crate time;

use std::os;
use std::num::{Float};
use std::sync::{Arc};
use std::thread;

use color::{ColorRGB};
use geometry::{Point3D, Vector3D, Direction3D, Ray3D};
use image::{PPMImage};
use material::{MaterialBuilder};
use scene::{Scene, SceneLight, Sphere, Plane, Camera};
use table::{Table, AsTable};

mod color;
mod geometry;
mod image;
mod material;
mod scene;
mod table;

static EXAMPLE_TO_RUN: u32 = 3;

fn main() {

    let start = time::precise_time_ns();

    let camera;
    let mut scene;

    let scene_setup_start = time::precise_time_ns();
    if EXAMPLE_TO_RUN == 1 {
        //----------------------------------------------------------------------
        // Scratchapixel Tutorial
        //----------------------------------------------------------------------
            
        let background_color = ColorRGB::from_rgb(2.0, 2.0, 2.0);
        scene = Scene::new(&background_color, 1.0, 5);

        let ground_sphere = Box::new(Sphere::new(&Point3D::from_xyz(0.0, -10004.0, 20.0), 10000.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.20, 0.20, 0.20))
            .diffuse(1.0)
            .specular(0.0)
            .shininess(0)
            .reflection(0.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        ));
        scene.add_object(ground_sphere);

        let sphere1 = Box::new(Sphere::new(&Point3D::from_xyz(0.0, 0.0, 20.0), 4.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(1.00, 0.32, 0.36))
            .diffuse(1.0)
            .specular(0.0)
            .shininess(0)
            .reflection(1.0)
            .refraction(0.5)
            .refractive_index(1.1)
            .to_material()
        ));
        scene.add_object(sphere1);

        let sphere2 = Box::new(Sphere::new(&Point3D::from_xyz(5.0, -1.0, 15.0), 2.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.90, 0.76, 0.46))
            .diffuse(1.0)
            .specular(0.0)
            .shininess(0)
            .reflection(1.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        ));
        scene.add_object(sphere2);

        let sphere3 = Box::new(Sphere::new(&Point3D::from_xyz(5.0, 0.0, 25.0), 3.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.65, 0.77, 0.97))
            .diffuse(1.0)
            .specular(0.0)
            .shininess(0)
            .reflection(1.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        ));
        scene.add_object(sphere3);

        let sphere4 = Box::new(Sphere::new(&Point3D::from_xyz(-5.5, 0.0, 15.0), 3.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.90, 0.90, 0.90))
            .diffuse(1.0)
            .specular(0.0)
            .shininess(0)
            .reflection(1.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        ));
        scene.add_object(sphere4);

        let light_source = Box::new(SceneLight::new(&Point3D::from_xyz(0.0, 20.0, 30.0), 3.0, &ColorRGB::from_rgb(3.0, 3.0, 3.0)));
        scene.add_light_source(light_source);

        let image_dimensions = (640, 480);
        let field_of_view: f32 = 30.0;
        camera = Camera::from_fov(image_dimensions, field_of_view, 1.0, Point3D::origin(), &Point3D::from_xyz(0.0, 0.0, 1.0));
        
    } else if EXAMPLE_TO_RUN == 2 {
        //----------------------------------------------------------------------
        // flipcode Tutorial, version 1 & version 2
        //----------------------------------------------------------------------

        scene = Scene::new(ColorRGB::black(), 1.0, 5);

        let ground_plane = Box::new(Plane::from_d_vector(4.4, &Vector3D::from_xyz(0.0, 1.0, 0.0), &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.4, 0.3, 0.3))
            .diffuse(1.0)
            .specular(0.0)
            .shininess(0)
            .reflection(0.0)
            .to_material()
        ));
        scene.add_object(ground_plane);

        let big_sphere = Box::new(Sphere::new(&Point3D::from_xyz(1.0, -0.8, 3.0), 2.5, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.7, 0.7, 0.7))
            .diffuse(0.2)
            .specular(0.8)
            .shininess(20)
            .reflection(0.6)
            .to_material()
        ));
        scene.add_object(big_sphere);

        let small_sphere = Box::new(Sphere::new(&Point3D::from_xyz(-5.5, -0.5, 7.0), 2.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.7, 0.7, 1.0))
            .diffuse(0.1)
            .specular(0.9)
            .shininess(20)
            .reflection(1.0)
            .to_material()
        ));
        scene.add_object(small_sphere);

        let light_source1 = Box::new(SceneLight::new(&Point3D::from_xyz(0.0, 5.0, 5.0), 0.1, &ColorRGB::from_rgb(0.6, 0.6, 0.6)));
        scene.add_light_source(light_source1);

        let light_source2 = Box::new(SceneLight::new(&Point3D::from_xyz(2.0, 5.0, 1.0), 0.1, &ColorRGB::from_rgb(0.7, 0.7, 0.9)));
        scene.add_light_source(light_source2);

        let image_dimensions = (800, 600);
        camera = Camera::from_dimensions(image_dimensions, (8.0, 6.0), 5.0, &Point3D::from_xyz(0.0, 0.0, -5.0), &Point3D::from_xyz(0.0, 0.0, 1.0));

    } else {
        //----------------------------------------------------------------------
        // flipcode Tutorial, version 3
        //----------------------------------------------------------------------

        scene = Scene::new(ColorRGB::black(), 1.0, 5);

        let ground_plane = Box::new(Plane::from_d_vector(4.4, &Vector3D::from_xyz(0.0, 1.0, 0.0), &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.4, 0.3, 0.3))
            .diffuse(1.0)
            .specular(0.8)
            .shininess(20)
            .reflection(0.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        ));
        scene.add_object(ground_plane);

        let big_sphere = Box::new(Sphere::new(&Point3D::from_xyz(2.0, 0.8, 3.0), 2.5, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.7, 0.7, 1.0))
            .diffuse(0.2)
            .specular(0.8)
            .shininess(20)
            .reflection(0.2)
            .refraction(0.8)
            .refractive_index(1.3)
            .to_material()
        ));
        scene.add_object(big_sphere);

        let small_sphere = Box::new(Sphere::new(&Point3D::from_xyz(-5.5, -0.5, 7.0), 2.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.7, 0.7, 1.0))
            .diffuse(0.1)
            .specular(0.8)
            .shininess(20)
            .reflection(0.5)
            .refraction(0.0)
            .refractive_index(1.3)
            .to_material()
        ));
        scene.add_object(small_sphere);

        let light_source1 = Box::new(SceneLight::new(&Point3D::from_xyz(0.0, 5.0, 5.0), 0.1, &ColorRGB::from_rgb(0.4, 0.4, 0.4)));
        scene.add_light_source(light_source1);

        let light_source2 = Box::new(SceneLight::new(&Point3D::from_xyz(-3.0, 5.0, 1.0), 0.1, &ColorRGB::from_rgb(0.6, 0.6, 0.8)));
        scene.add_light_source(light_source2);

        let extra_sphere = Box::new(Sphere::new(&Point3D::from_xyz(-1.5, -3.8, 1.0), 1.5, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(1.0, 0.4, 0.4))
            .diffuse(0.2)
            .specular(0.8)
            .shininess(20)
            .reflection(0.0)
            .refraction(0.8)
            .refractive_index(1.5)
            .to_material()
        ));
        scene.add_object(extra_sphere);

        let back_plane = Box::new(Plane::from_d_vector(12.0, &Vector3D::from_xyz(0.4, 0.0, -1.0), &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.5, 0.3, 0.5))
            .diffuse(0.6)
            .specular(0.0)
            .shininess(0)
            .reflection(0.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        ));
        scene.add_object(back_plane);

        let ceiling_plane = Box::new(Plane::from_d_vector(7.4, &Vector3D::from_xyz(0.0, -1.0, 0.0), &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.4, 0.7, 0.7))
            .diffuse(0.5)
            .specular(0.0)
            .shininess(0)
            .reflection(0.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        ));
        scene.add_object(ceiling_plane);

        for x in 0u32..8 {
            for y in 0u32..7 {
                let grid_sphere = Box::new(Sphere::new(&Point3D::from_xyz(-4.5 + (x as f32) * 1.5, -4.3 + (y as f32) * 1.5, 10.0), 0.3, &MaterialBuilder::new()
                    .color(&ColorRGB::from_rgb(0.3, 1.0, 0.4))
                    .diffuse(0.6)
                    .specular(0.6)
                    .shininess(20)
                    .reflection(0.0)
                    .refraction(0.0)
                    .refractive_index(0.0)
                    .to_material()
                ));
                scene.add_object(grid_sphere);
            }
        }

        let image_dimensions = (800, 600);
        camera = Camera::from_dimensions(image_dimensions, (8.0, 6.0), 5.0, &Point3D::from_xyz(0.0, 0.0, -5.0), &Point3D::from_xyz(0.0, 0.0, 1.0));
    }

    let scene_setup_end = time::precise_time_ns();
    let elapsed = (scene_setup_end - scene_setup_start) / 1000000;
    println!("Scene Setup     : {}", elapsed);

    let pixel_table = render(Arc::new(scene), Arc::new(camera));

    let image_saving_start = time::precise_time_ns();

    let image = PPMImage::new(&*format!("example{}.ppm", EXAMPLE_TO_RUN));
    let result = image.save(&pixel_table);

    let image_saving_end = time::precise_time_ns();
    let elapsed = (image_saving_end - image_saving_start) / 1000000;
    println!("Image Saving    : {}", elapsed);
    
    match result {
        Ok(_) => println!("Image rendered successfully"),
        Err(e) => println!("Image rendering failed:\n{}", e)
    }

    //--------------------------------------------------------------------------

    let end = time::precise_time_ns();
    let elapsed = (end - start) / 1000000;
    println!("Elapsed time: {}", elapsed);
}
    
fn render(scene: Arc<Scene>, camera: Arc<Camera>) -> Table<ColorRGB> {
    let dimensions = camera.get_image_dimensions();
    let (width, height) = dimensions;

    let mut pixel_table = Table::from_elem(dimensions, ColorRGB::black().clone());

    // Initial Pixel Coloring
    // let intital_coloring_start = time::precise_time_ns();
    // for (index, value) in pixel_table.iter_mut().enumerate_2d() {
    //     let ray = camera.get_primary_ray(index);
    //     let result = scene.trace(&ray, 0);
        
    //     let result_color = ColorRGB::from_rgb(
    //         result.color.red.min(1.0),
    //         result.color.green.min(1.0),
    //         result.color.blue.min(1.0)
    //     );
    //     *value = result_color;
    // }
    // let initial_coloring_end = time::precise_time_ns();

    let thread_setup_start = time::precise_time_ns();
    let num_threads = os::num_cpus();

    let total_pixels = width * height;
    let pixels_per_thread = if total_pixels % num_threads > 0 {
        total_pixels / num_threads + 1
    } else {
        total_pixels / num_threads
    };

    // Initial Pixel Coloring
    let initial_coloring_threads = (0..num_threads).map(|thread_index| {
        let local_camera = camera.clone();
        let local_scene = scene.clone();
        thread::scoped(move|| {
            let start_index = pixels_per_thread * thread_index;

            let num_pixels = if thread_index != num_threads-1 {
                pixels_per_thread
            } else {
                total_pixels - start_index
            };

            (0..num_pixels).as_table(dimensions).enumerate_2d_from_index(start_index).map(|(index, _)| {
                let ray = local_camera.get_primary_ray(index);
                let result = local_scene.trace(&ray, 0);
                
                ColorRGB::from_rgb(
                    result.color.red.min(1.0),
                    result.color.green.min(1.0),
                    result.color.blue.min(1.0)
                )
            }).collect::<Vec<_>>()
        })
    }).collect::<Vec<_>>();
    let thread_setup_end = time::precise_time_ns();

    let thread_waiting_start = time::precise_time_ns();
    let initial_coloring = initial_coloring_threads.into_iter().flat_map(|f| f.join().into_iter()).collect::<Vec<_>>();
    let thread_waiting_end = time::precise_time_ns();

    // Collect the colored pixels back into the original table.
    let pixel_combining_start = time::precise_time_ns();
    for (pixel, color) in pixel_table.iter_mut().zip(initial_coloring.iter()) {
        *pixel = color.clone();
    }
    let pixel_combining_end = time::precise_time_ns();
    
    // Edge Detection
    let edge_detection_start = time::precise_time_ns();
    let mut is_edge = Table::from_elem(dimensions, false);
    for (index, value) in is_edge.iter_mut().enumerate_2d() {
        let (row, column) = index;
        if row != 0 && column != 0 && row != height-1 && column != width-1 {
            let ref p1 = pixel_table[(row - 1, column - 1)];
            let ref p2 = pixel_table[(row - 1, column)];
            let ref p3 = pixel_table[(row - 1, column + 1)];
            let ref p4 = pixel_table[(row, column - 1)];
            let ref p6 = pixel_table[(row, column + 1)];
            let ref p7 = pixel_table[(row + 1, column - 1)];
            let ref p8 = pixel_table[(row + 1, column)];
            let ref p9 = pixel_table[(row + 1, column + 1)];

            let r = calculate_gradient(p1.red, p2.red, p3.red, p4.red, p6.red, p7.red, p8.red, p9.red);
            let g = calculate_gradient(p1.green, p2.green, p3.green, p4.green, p6.green, p7.green, p8.green, p9.green);
            let b = calculate_gradient(p1.blue, p2.blue, p3.blue, p4.blue, p6.blue, p7.blue, p8.blue, p9.blue);

            if (r + b + g) > 0.5 {
                *value = true;
            } else {
                *value = false;
            }
        }
    }
    let edge_detection_end = time::precise_time_ns();

    // Anti-aliasing
    let anti_aliasing_start = time::precise_time_ns();
    let sub_width = 3;
    let sub_height = 3;
    let sub_size = (sub_width * sub_height) as f32;
    let mut sub_rays = Table::from_elem((sub_width, sub_height), Ray3D::new(Point3D::origin(), Direction3D::unit_x()));
    for (index, value) in pixel_table.iter_mut().enumerate_2d() {
        if is_edge[index] {
            let mut pixel_color = ColorRGB::black().clone();
            
            camera.get_sub_rays(index, &mut sub_rays);
            for sub_ray in &sub_rays {
                let result = scene.trace(sub_ray, 0);
                
                pixel_color = ColorRGB::from_rgb(
                    pixel_color.red + result.color.red / sub_size,
                    pixel_color.green + result.color.green / sub_size,
                    pixel_color.blue + result.color.blue / sub_size
                );
            }
            *value = pixel_color;
        }
    }
    let anti_aliasing_end = time::precise_time_ns();

    // let elapsed = (initial_coloring_end - intital_coloring_start) / 1000000;
    // println!("Initial Coloring: {}", elapsed);
    let elapsed = (thread_setup_end - thread_setup_start) / 1000000;
    println!("Thread Setup    : {}", elapsed);
    let elapsed = (thread_waiting_end - thread_waiting_start) / 1000000;
    println!("Thread Waiting  : {}", elapsed);
    let elapsed = (pixel_combining_end - pixel_combining_start) / 1000000;
    println!("Pixel Combining : {}", elapsed);
    let elapsed = (edge_detection_end - edge_detection_start) / 1000000;
    println!("Edge Detection  : {}", elapsed);
    let elapsed = (anti_aliasing_end - anti_aliasing_start) / 1000000;
    println!("Anti-aliasing   : {}", elapsed);

    pixel_table
}

fn calculate_gradient(p1: f32, p2: f32, p3: f32, p4: f32, p6: f32, p7: f32, p8: f32, p9: f32) -> f32
{
    let gx = (p3 + 2.0 * p6 + p9) - (p1 + 2.0 * p4 + p7);
    let gy = (p1 + 2.0 * p2 + p3) - (p7 + 2.0 * p8 + p9);
    (gx*gx + gy*gy).sqrt()
}

extern crate time;

use geometry::{Point3D, Vector3D};
use color::{ColorRGB};
use material::{MaterialBuilder};

use sphere::{Sphere};
use plane::{Plane};
use scene_light::{SceneLight};

use table::{Table};
use camera::{Camera};
use ppm_image::{PPMImage};

use scene::{Scene};

mod geometry;
mod color;
mod material;

mod scene_object;
mod sphere;
mod plane;
mod scene_light;

mod table;
mod camera;
mod ppm_image;

mod scene;

static EXAMPLE_TO_RUN: uint = 3;

fn main() {

    let start = time::precise_time_ns();
    let result;

    if EXAMPLE_TO_RUN == 1 {
        //----------------------------------------------------------------------
        // Scratchapixel Tutorial
        //----------------------------------------------------------------------
    
        let dimensions = (640, 480);
        let field_of_view: f32 = 30.0;
        
        let background_color = ColorRGB::from_rgb(2.0, 2.0, 2.0);
        let mut scene = Scene::new(&background_color, 1.0, 5);

        let ground_sphere = Sphere::new(&Point3D::from_xyz(0.0, -10004.0, 20.0), 10000.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.20, 0.20, 0.20))
            .diffuse(1.0)
            .specular(0.0)
            .shininess(0)
            .reflection(0.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        );
        scene.add_object(&ground_sphere);

        let sphere1 = Sphere::new(&Point3D::from_xyz(0.0, 0.0, 20.0), 4.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(1.00, 0.32, 0.36))
            .diffuse(1.0)
            .specular(0.0)
            .shininess(0)
            .reflection(1.0)
            .refraction(0.5)
            .refractive_index(1.1)
            .to_material()
        );
        scene.add_object(&sphere1);

        let sphere2 = Sphere::new(&Point3D::from_xyz(5.0, -1.0, 15.0), 2.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.90, 0.76, 0.46))
            .diffuse(1.0)
            .specular(0.0)
            .shininess(0)
            .reflection(1.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        );
        scene.add_object(&sphere2);

        let sphere3 = Sphere::new(&Point3D::from_xyz(5.0, 0.0, 25.0), 3.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.65, 0.77, 0.97))
            .diffuse(1.0)
            .specular(0.0)
            .shininess(0)
            .reflection(1.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        );
        scene.add_object(&sphere3);

        let sphere4 = Sphere::new(&Point3D::from_xyz(-5.5, 0.0, 15.0), 3.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.90, 0.90, 0.90))
            .diffuse(1.0)
            .specular(0.0)
            .shininess(0)
            .reflection(1.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        );
        scene.add_object(&sphere4);

        let light_source = SceneLight::new(&Point3D::from_xyz(0.0, 20.0, 30.0), 3.0, &ColorRGB::from_rgb(3.0, 3.0, 3.0));
        scene.add_light_source(&light_source);

        let mut pixel_table = Table::from_elem(dimensions, *ColorRGB::black());
        let camera = Camera::from_fov(dimensions, field_of_view, 1.0, Point3D::origin(), &Point3D::from_xyz(0.0, 0.0, 1.0));
        scene.render(&camera, &mut pixel_table);

        let image = PPMImage::new("example1.ppm");
        result = image.save(&pixel_table);
        
    } else if EXAMPLE_TO_RUN == 2 {
        //----------------------------------------------------------------------
        // flipcode Tutorial, version 1 & version 2
        //----------------------------------------------------------------------

        let dimensions = (800, 600);

        let mut scene = Scene::new(ColorRGB::black(), 1.0, 5);

        let ground_plane = Plane::from_d_vector(4.4, &Vector3D::from_xyz(0.0, 1.0, 0.0), &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.4, 0.3, 0.3))
            .diffuse(1.0)
            .specular(0.0)
            .shininess(0)
            .reflection(0.0)
            .to_material()
        );
        scene.add_object(&ground_plane);

        let big_sphere = Sphere::new(&Point3D::from_xyz(1.0, -0.8, 3.0), 2.5, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.7, 0.7, 0.7))
            .diffuse(0.2)
            .specular(0.8)
            .shininess(20)
            .reflection(0.6)
            .to_material()
        );
        scene.add_object(&big_sphere);

        let small_sphere = Sphere::new(&Point3D::from_xyz(-5.5, -0.5, 7.0), 2.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.7, 0.7, 1.0))
            .diffuse(0.1)
            .specular(0.9)
            .shininess(20)
            .reflection(1.0)
            .to_material()
        );
        scene.add_object(&small_sphere);

        let light_source1 = SceneLight::new(&Point3D::from_xyz(0.0, 5.0, 5.0), 0.1, &ColorRGB::from_rgb(0.6, 0.6, 0.6));
        scene.add_light_source(&light_source1);

        let light_source2 = SceneLight::new(&Point3D::from_xyz(2.0, 5.0, 1.0), 0.1, &ColorRGB::from_rgb(0.7, 0.7, 0.9));
        scene.add_light_source(&light_source2);

        let mut pixel_table = Table::from_elem(dimensions, *ColorRGB::black());
        let camera = Camera::from_dimensions(dimensions, (8.0, 6.0), 5.0, &Point3D::from_xyz(0.0, 0.0, -5.0), &Point3D::from_xyz(0.0, 0.0, 1.0));
        scene.render(&camera, &mut pixel_table);

        let image = PPMImage::new("example2.ppm");
        result = image.save(&pixel_table);
    } else {
        //----------------------------------------------------------------------
        // flipcode Tutorial, version 3
        //----------------------------------------------------------------------
    
        let dimensions = (800, 600);

        let mut scene = Scene::new(ColorRGB::black(), 1.0, 5);

        let ground_plane = Plane::from_d_vector(4.4, &Vector3D::from_xyz(0.0, 1.0, 0.0), &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.4, 0.3, 0.3))
            .diffuse(1.0)
            .specular(0.8)
            .shininess(20)
            .reflection(0.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        );
        scene.add_object(&ground_plane);

        let big_sphere = Sphere::new(&Point3D::from_xyz(2.0, 0.8, 3.0), 2.5, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.7, 0.7, 1.0))
            .diffuse(0.2)
            .specular(0.8)
            .shininess(20)
            .reflection(0.2)
            .refraction(0.8)
            .refractive_index(1.3)
            .to_material()
        );
        scene.add_object(&big_sphere);

        let small_sphere = Sphere::new(&Point3D::from_xyz(-5.5, -0.5, 7.0), 2.0, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.7, 0.7, 1.0))
            .diffuse(0.1)
            .specular(0.8)
            .shininess(20)
            .reflection(0.5)
            .refraction(0.0)
            .refractive_index(1.3)
            .to_material()
        );
        scene.add_object(&small_sphere);

        let light_source1 = SceneLight::new(&Point3D::from_xyz(0.0, 5.0, 5.0), 0.1, &ColorRGB::from_rgb(0.4, 0.4, 0.4));
        scene.add_light_source(&light_source1);

        let light_source2 = SceneLight::new(&Point3D::from_xyz(-3.0, 5.0, 1.0), 0.1, &ColorRGB::from_rgb(0.6, 0.6, 0.8));
        scene.add_light_source(&light_source2);

        let extra_sphere = Sphere::new(&Point3D::from_xyz(-1.5, -3.8, 1.0), 1.5, &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(1.0, 0.4, 0.4))
            .diffuse(0.2)
            .specular(0.8)
            .shininess(20)
            .reflection(0.0)
            .refraction(0.8)
            .refractive_index(1.5)
            .to_material()
        );
        scene.add_object(&extra_sphere);

        let back_plane = Plane::from_d_vector(12.0, &Vector3D::from_xyz(0.4, 0.0, -1.0), &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.5, 0.3, 0.5))
            .diffuse(0.6)
            .specular(0.0)
            .shininess(0)
            .reflection(0.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        );
        scene.add_object(&back_plane);

        let ceiling_plane = Plane::from_d_vector(7.4, &Vector3D::from_xyz(0.0, -1.0, 0.0), &MaterialBuilder::new()
            .color(&ColorRGB::from_rgb(0.4, 0.7, 0.7))
            .diffuse(0.5)
            .specular(0.0)
            .shininess(0)
            .reflection(0.0)
            .refraction(0.0)
            .refractive_index(0.0)
            .to_material()
        );
        scene.add_object(&ceiling_plane);

        let mut grid_spheres: Vec<Sphere> = Vec::new();
        for x in range::<uint>(0, 8) {
            for y in range::<uint>(0, 7) {
                grid_spheres.push(Sphere::new(&Point3D::from_xyz(-4.5 + (x as f32) * 1.5, -4.3 + (y as f32) * 1.5, 10.0), 0.3, &MaterialBuilder::new()
                    .color(&ColorRGB::from_rgb(0.3, 1.0, 0.4))
                    .diffuse(0.6)
                    .specular(0.6)
                    .shininess(20)
                    .reflection(0.0)
                    .refraction(0.0)
                    .refractive_index(0.0)
                    .to_material()
                ));
            }
        }
        for grid_sphere in grid_spheres.iter() {
            scene.add_object(grid_sphere);
        }

        let mut pixel_table = Table::from_elem(dimensions, *ColorRGB::black());
        let camera = Camera::from_dimensions(dimensions, (8.0, 6.0), 5.0, &Point3D::from_xyz(0.0, 0.0, -5.0), &Point3D::from_xyz(0.0, 0.0, 1.0));
        scene.render(&camera, &mut pixel_table);

        let image = PPMImage::new("example3.ppm");
        result = image.save(&pixel_table);
    }
    
    match result {
        Ok(_) => println!("Image rendered successfully"),
        Err(e) => println!("Image rendering failed:\n{}", e)
    }

    //--------------------------------------------------------------------------

    let end = time::precise_time_ns();
    let elapsed = (end - start) / 1000000;
    println!("Elapsed time: {}", elapsed);
}
use std::fs::{OpenOptions};
use std::io::{Write, Result};
use std::num::{Float};

use color::{ColorRGB};
use table::{Table};

#[derive(Debug)]
pub struct PPMImage {
    file_name: String
}

impl PPMImage {
    pub fn new(file_name: &str) -> PPMImage {
        PPMImage {
            file_name: file_name.to_string()
        }
    }
    
    pub fn save(&self, image: &Table<ColorRGB>) -> Result<()> {
        let file_path = Path::new(&self.file_name);

        let mut file = try!(OpenOptions::new().create(true).write(true).open(&file_path));

        let (width, height) = image.get_dimensions();

        try!(writeln!(&mut file, "P6"));
        try!(writeln!(&mut file, "{} {}", width, height));
        try!(writeln!(&mut file, "255"));

        for pixel in image {                
            let red = convert_to_u8(pixel.red);
            let green = convert_to_u8(pixel.green);
            let blue = convert_to_u8(pixel.blue);
            
            let pixel_bytes = vec![red, green, blue];
            try!(file.write(&pixel_bytes));
        }
        Ok(())
    }
}

fn convert_to_u8(value: f32) -> u8 {
    (value * 255.0).min(255.0).max(0.0) as u8
}

use std::io::{File, Open, Write, IoResult};
use std::num::{Float};

use color::{ColorRGB};
use table::{Table};

#[derive(Show)]
pub struct PPMImage {
    file_name: String
}

impl PPMImage {
    pub fn new(file_name: &str) -> PPMImage {
        PPMImage {
            file_name: file_name.to_string()
        }
    }
    
    pub fn save(&self, image: &Table<ColorRGB>) -> IoResult<()> {
        let file_path = Path::new(&self.file_name);
        let mut file = try!(File::open_mode(&file_path, Open, Write));

        let (width, height) = image.get_dimensions();

        try!(file.write_line("P6"));
        try!(file.write_uint(width));
        try!(file.write_str(" "));
        try!(file.write_uint(height));
        try!(file.write_line(""));
        try!(file.write_line("255"));

        for pixel in image.iter() {                
            let red = convert_to_u8(pixel.red);
            let green = convert_to_u8(pixel.green);
            let blue = convert_to_u8(pixel.blue);
            
            try!(file.write_u8(red));
            try!(file.write_u8(green));
            try!(file.write_u8(blue));
        }
        Ok(())
    }
}

fn convert_to_u8(value: f32) -> u8 {
    (value * 255.0).min(255.0).max(0.0) as u8
}

#![allow(dead_code)]

pub struct ColorRGB {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

static COLOR_RGB_WHITE: ColorRGB = ColorRGB {red: 1.0, green: 1.0, blue: 1.0};
static COLOR_RGB_BLACK: ColorRGB = ColorRGB {red: 0.0, green: 0.0, blue: 0.0};
static COLOR_RGB_RED: ColorRGB = ColorRGB {red: 1.0, green: 0.0, blue: 0.0};
static COLOR_RGB_GREEN: ColorRGB = ColorRGB {red: 0.0, green: 1.0, blue: 0.0};
static COLOR_RGB_BLUE: ColorRGB = ColorRGB {red: 0.0, green: 0.0, blue: 1.0};

impl ColorRGB {
    //pub fn new() -> ColorRGB {
    //    ColorRGB::from_rgb(0.0, 0.0, 0.0)
    //}
    
    pub fn white() -> &'static ColorRGB {
        &COLOR_RGB_WHITE
    }
    
    pub fn black() -> &'static ColorRGB {
        &COLOR_RGB_BLACK
    }
    
    pub fn red() -> &'static ColorRGB {
        &COLOR_RGB_RED
    }
    
    pub fn green() -> &'static ColorRGB {
        &COLOR_RGB_GREEN
    }
    
    pub fn blue() -> &'static ColorRGB {
        &COLOR_RGB_BLUE
    }
    
    pub fn from_rgb(red: f32, green:f32, blue:f32) -> ColorRGB {
        ColorRGB {
            red: red, 
            green: green, 
            blue: blue
        }
    }
}

impl Add<ColorRGB, ColorRGB> for ColorRGB {
    fn add(&self, other: &ColorRGB) -> ColorRGB {
        ColorRGB::from_rgb(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue
        )
    }
}

trait MulColorRGB {
    fn mul(&self, lhs: &ColorRGB) -> ColorRGB;
}

impl<T: MulColorRGB> Mul<T, ColorRGB> for ColorRGB {
    fn mul(&self, other: &T) -> ColorRGB {
        other.mul(self)
    }
}

impl MulColorRGB for f32 {
    fn mul(&self, lhs: &ColorRGB) -> ColorRGB {
        ColorRGB::from_rgb(
            lhs.red * *self,
            lhs.green * *self,
            lhs.blue * *self
       )  
    }
}

impl MulColorRGB for ColorRGB {
    fn mul(&self, lhs: &ColorRGB) -> ColorRGB {
        ColorRGB::from_rgb(
            lhs.red * self.red,
            lhs.green * self.green,
            lhs.blue * self.blue
       )  
    }
}

impl Clone for ColorRGB {
    fn clone(&self) -> ColorRGB {
        ColorRGB {
            red: self.red,
            green: self.green,
            blue: self.blue
        }
    }
    
    fn clone_from(&mut self, source: &ColorRGB) {
        self.red = source.red;
        self.green = source.green;
        self.blue = source.blue;
    }
}

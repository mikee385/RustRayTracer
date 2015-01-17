use std::ops::{Add, Mul};

#[derive(PartialEq, PartialOrd, Clone, Show)]
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

impl<'a, 'b> Add<&'a ColorRGB> for &'b ColorRGB {
    type Output = ColorRGB;

    fn add(self, other: &ColorRGB) -> ColorRGB {
        ColorRGB::from_rgb(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue
        )
    }
}

impl<'a> Add<&'a ColorRGB> for ColorRGB {
    type Output = ColorRGB;
    
    fn add(self, other: &ColorRGB) -> ColorRGB {
        &self + other
    }
}

impl<'a> Add<ColorRGB> for &'a ColorRGB {
    type Output = ColorRGB;
    
    fn add(self, other: ColorRGB) -> ColorRGB {
        self + &other
    }
}

impl Add<ColorRGB> for ColorRGB {
    type Output = ColorRGB;
    
    fn add(self, other: ColorRGB) -> ColorRGB {
        &self + &other
    }
}

impl<'a> Mul<f32> for &'a ColorRGB {
    type Output = ColorRGB;
    
    fn mul(self, scale: f32) -> ColorRGB {
        ColorRGB::from_rgb(
            self.red * scale,
            self.green * scale,
            self.blue * scale
       )
    }
}

impl Mul<f32> for ColorRGB {
    type Output = ColorRGB;
    
    fn mul(self, scale: f32) -> ColorRGB {
        &self * scale
    }
}

impl<'a, 'b> Mul<&'a ColorRGB> for &'b ColorRGB {
    type Output = ColorRGB;
    
    fn mul(self, other: &ColorRGB) -> ColorRGB {
        ColorRGB::from_rgb(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue
       )  
    }
}

impl<'a> Mul<&'a ColorRGB> for ColorRGB {
    type Output = ColorRGB;
    
    fn mul(self, other: &ColorRGB) -> ColorRGB {
        &self * other
    }
}

impl<'a> Mul<ColorRGB> for &'a ColorRGB {
    type Output = ColorRGB;
    
    fn mul(self, other: ColorRGB) -> ColorRGB {
        self * &other
    }
}

impl Mul<ColorRGB> for ColorRGB {
    type Output = ColorRGB;
    
    fn mul(self, other: ColorRGB) -> ColorRGB {
        &self * &other
    }
}

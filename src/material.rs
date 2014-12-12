use color::ColorRGB;

#[deriving(PartialEq, PartialOrd, Copy, Clone, Show)]
pub struct Material {
    pub color: ColorRGB,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: u32,
    pub reflection:f32,
    pub refraction: f32,
    pub refractive_index: f32
}

impl Material {
    pub fn new(color: &ColorRGB) -> Material {
        Material {
            color: *color,
            diffuse: 1.0,
            specular: 0.0,
            shininess: 0,
            reflection: 0.0,
            refraction: 0.0,
            refractive_index: 0.0
        }
    }
}

pub struct MaterialBuilder {
    color: ColorRGB,
    diffuse: f32,
    specular: f32,
    shininess: u32,
    reflection: f32,
    refraction: f32,
    refractive_index: f32
}

impl MaterialBuilder {

    pub fn new() -> MaterialBuilder {
        MaterialBuilder {
            color: *ColorRGB::white(),
            diffuse: 1.0,
            specular: 0.0,
            shininess: 0,
            reflection: 0.0,
            refraction: 0.0,
            refractive_index: 0.0
        }
    }

    pub fn color(&mut self, color: &ColorRGB) -> &mut MaterialBuilder {
        self.color = *color;
        self
    }

    pub fn diffuse(&mut self, diffuse: f32) -> &mut MaterialBuilder {
        self.diffuse = diffuse;
        self
    }

    pub fn specular(&mut self, specular: f32) -> &mut MaterialBuilder {
        self.specular = specular;
        self
    }

    pub fn shininess(&mut self, shininess: u32) -> &mut MaterialBuilder {
        self.shininess = shininess;
        self
    }
    
    pub fn reflection(&mut self, reflection: f32) -> &mut MaterialBuilder {
        self.reflection = reflection;
        self
    }
    
    pub fn refraction(&mut self, refraction: f32) -> &mut MaterialBuilder {
        self.refraction = refraction;
        self
    }
    
    pub fn refractive_index(&mut self, refractive_index: f32) -> &mut MaterialBuilder {
        self.refractive_index = refractive_index;
        self
    }
    
    pub fn to_material(&self) -> Material {
        Material {
            color: self.color, 
            diffuse: self.diffuse, 
            specular: self.specular, 
            shininess: self.shininess, 
            reflection: self.reflection, 
            refraction: self.refraction, 
            refractive_index: self.refractive_index
        }
    }
}

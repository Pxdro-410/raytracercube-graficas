use crate::color::Color;
use crate::texture::Texture;
use nalgebra_glm::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct Material {
    pub diffuse: Color,
    pub texture: Option<Arc<Texture>>,
}

impl Material {
    pub fn new(diffuse: Color) -> Self {
        Material {
            diffuse,
            texture: None,
        }
    }

    pub fn with_texture(texture: Arc<Texture>) -> Self {
        Material {
            diffuse: Color::new(255, 255, 255),
            texture: Some(texture),
        }
    }

    pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
        if let Some(ref tex) = self.texture {
            tex.get_color(u, v)
        } else {
            self.diffuse
        }
    }
}

#[derive(Clone)]
pub struct Intersect {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub u: f32,
    pub v: f32,
    pub material: Material,
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
}

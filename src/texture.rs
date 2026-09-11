use crate::color::Color;
use image::GenericImageView;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Texture {
    pub fn new(width: u32, height: u32, pixels: Vec<Color>) -> Self {
        Texture {
            width,
            height,
            pixels,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, image::ImageError> {
        let img = image::open(path)?;
        let (width, height) = img.dimensions();
        let rgba = img.to_rgba8();

        let mut pixels = Vec::with_capacity((width * height) as usize);
        for pixel in rgba.pixels() {
            pixels.push(Color::new(pixel[0], pixel[1], pixel[2]));
        }

        Ok(Texture {
            width,
            height,
            pixels,
        })
    }

    /// Genera una textura predeterminada (cuadrícula estilizada en verde aqua)
    /// si aún no se ha colocado una imagen en la carpeta assets/.
    pub fn default_grid() -> Self {
        let width = 64;
        let height = 64;
        let mut pixels = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                let is_border = x < 2 || x >= width - 2 || y < 2 || y >= height - 2;
                let is_cross = (x >= 30 && x <= 33) || (y >= 30 && y <= 33);
                let checker = ((x / 16) + (y / 16)) % 2 == 0;

                let color = if is_border || is_cross {
                    Color::new(25, 90, 80)
                } else if checker {
                    Color::new(60, 230, 205)
                } else {
                    Color::new(40, 185, 165)
                };
                pixels.push(color);
            }
        }

        Texture {
            width,
            height,
            pixels,
        }
    }

    /// Muestrea el color correspondiente a las coordenadas UV [0.0, 1.0]
    pub fn get_color(&self, u: f32, v: f32) -> Color {
        if self.width == 0 || self.height == 0 || self.pixels.is_empty() {
            return Color::new(255, 255, 255);
        }

        let u = u.rem_euclid(1.0);
        let v = v.rem_euclid(1.0);

        let x = ((u * self.width as f32) as u32).min(self.width - 1);
        let y = ((v * self.height as f32) as u32).min(self.height - 1);

        let index = (y * self.width + x) as usize;
        self.pixels.get(index).copied().unwrap_or(Color::new(255, 255, 255))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_grid_sampling() {
        let texture = Texture::default_grid();
        assert_eq!(texture.width, 64);
        assert_eq!(texture.height, 64);

        let c1 = texture.get_color(0.5, 0.5);
        let c2 = texture.get_color(1.5, 2.5); // Prueba de envoltura / wrap
        assert_eq!(c1.to_hex(), c2.to_hex());
    }
}

use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::Vec3;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub top_bottom_material: Material,
    pub sides_material: Material,
}

impl Cube {
    pub fn new(center: Vec3, size: f32, material: Material) -> Self {
        let half = size / 2.0;
        Cube {
            min: center - Vec3::new(half, half, half),
            max: center + Vec3::new(half, half, half),
            top_bottom_material: material.clone(),
            sides_material: material,
        }
    }

    pub fn with_face_materials(
        center: Vec3,
        size: f32,
        top_bottom_material: Material,
        sides_material: Material,
    ) -> Self {
        let half = size / 2.0;
        Cube {
            min: center - Vec3::new(half, half, half),
            max: center + Vec3::new(half, half, half),
            top_bottom_material,
            sides_material,
        }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let mut t_near = f32::NEG_INFINITY;
        let mut t_far = f32::INFINITY;
        let mut hit_normal = Vec3::zeros();

        for i in 0..3 {
            let origin = ray_origin[i];
            let dir = ray_direction[i];
            let min_val = self.min[i];
            let max_val = self.max[i];

            if dir.abs() < 1e-6 {
                if origin < min_val || origin > max_val {
                    return None;
                }
            } else {
                let mut t0 = (min_val - origin) / dir;
                let mut t1 = (max_val - origin) / dir;

                let mut n0 = Vec3::zeros();
                n0[i] = -1.0;
                let mut n1 = Vec3::zeros();
                n1[i] = 1.0;

                if t0 > t1 {
                    std::mem::swap(&mut t0, &mut t1);
                    std::mem::swap(&mut n0, &mut n1);
                }

                if t0 > t_near {
                    t_near = t0;
                    hit_normal = n0;
                }

                if t1 < t_far {
                    t_far = t1;
                }

                if t_near > t_far || t_far < 0.0 {
                    return None;
                }
            }
        }

        let t = if t_near > 1e-4 {
            t_near
        } else if t_far > 1e-4 {
            t_far
        } else {
            return None;
        };

        let point = ray_origin + ray_direction * t;

        let sx = self.max.x - self.min.x;
        let sy = self.max.y - self.min.y;
        let sz = self.max.z - self.min.z;

        // Mapeo UV y asignación de material según la cara (lados vs arriba/abajo)
        let (u, v, material) = if hit_normal.x > 0.5 {
            // Cara derecha (+X)
            ((self.max.z - point.z) / sz, (self.max.y - point.y) / sy, &self.sides_material)
        } else if hit_normal.x < -0.5 {
            // Cara izquierda (-X)
            ((point.z - self.min.z) / sz, (self.max.y - point.y) / sy, &self.sides_material)
        } else if hit_normal.y > 0.5 {
            // Cara superior (+Y)
            ((point.x - self.min.x) / sx, (point.z - self.min.z) / sz, &self.top_bottom_material)
        } else if hit_normal.y < -0.5 {
            // Cara inferior (-Y)
            ((point.x - self.min.x) / sx, (self.max.z - point.z) / sz, &self.top_bottom_material)
        } else if hit_normal.z > 0.5 {
            // Cara frontal (+Z)
            ((point.x - self.min.x) / sx, (self.max.y - point.y) / sy, &self.sides_material)
        } else {
            // Cara trasera (-Z)
            ((self.max.x - point.x) / sx, (self.max.y - point.y) / sy, &self.sides_material)
        };

        Some(Intersect {
            point,
            normal: hit_normal,
            distance: t,
            u: u.clamp(0.0, 1.0),
            v: v.clamp(0.0, 1.0),
            material: material.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_cube_intersection_front_and_uv() {
        let material = Material::new(Color::new(255, 0, 0));
        let cube = Cube::new(Vec3::new(0.0, 0.0, 0.0), 2.0, material);

        let ray_origin = Vec3::new(0.0, 0.0, 5.0);
        let ray_direction = Vec3::new(0.0, 0.0, -1.0);

        let hit = cube.ray_intersect(&ray_origin, &ray_direction).unwrap();
        assert!((hit.distance - 4.0).abs() < 1e-4);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(hit.point, Vec3::new(0.0, 0.0, 1.0));
        assert!((hit.u - 0.5).abs() < 1e-4);
        assert!((hit.v - 0.5).abs() < 1e-4);
    }

    #[test]
    fn test_cube_miss() {
        let material = Material::new(Color::new(255, 0, 0));
        let cube = Cube::new(Vec3::new(0.0, 0.0, 0.0), 2.0, material);

        let ray_origin = Vec3::new(0.0, 5.0, 5.0);
        let ray_direction = Vec3::new(0.0, 0.0, -1.0);

        assert!(cube.ray_intersect(&ray_origin, &ray_direction).is_none());
    }
}

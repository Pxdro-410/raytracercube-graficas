use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::Vec3;

pub struct Plane {
    pub y: f32,
    pub half_size: f32,
    pub material: Material,
}

impl Plane {
    pub fn new(y: f32, half_size: f32, material: Material) -> Self {
        Plane {
            y,
            half_size,
            material,
        }
    }
}

impl RayIntersect for Plane {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        if ray_direction.y.abs() < 1e-6 {
            return None;
        }

        let t = (self.y - ray_origin.y) / ray_direction.y;
        if t <= 1e-4 {
            return None;
        }

        let point = ray_origin + ray_direction * t;

        // Limitar las dimensiones del piso para que no sea infinito
        if point.x.abs() > self.half_size || point.z.abs() > self.half_size {
            return None;
        }

        let normal = if ray_origin.y > self.y {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(0.0, -1.0, 0.0)
        };

        let u = (point.x + self.half_size) / (2.0 * self.half_size);
        let v = (point.z + self.half_size) / (2.0 * self.half_size);

        Some(Intersect {
            point,
            normal,
            distance: t,
            u,
            v,
            material: self.material.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_plane_hit_within_bounds() {
        let material = Material::new(Color::new(150, 150, 150));
        let plane = Plane::new(-1.0, 3.0, material);
        let ray_origin = Vec3::new(0.0, 1.0, 0.0);
        let ray_direction = Vec3::new(0.0, -1.0, 0.0);

        let hit = plane.ray_intersect(&ray_origin, &ray_direction).unwrap();
        assert!((hit.distance - 2.0).abs() < 1e-4);
        assert_eq!(hit.normal, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(hit.point, Vec3::new(0.0, -1.0, 0.0));
    }

    #[test]
    fn test_plane_miss_outside_bounds() {
        let material = Material::new(Color::new(150, 150, 150));
        let plane = Plane::new(-1.0, 2.0, material);
        let ray_origin = Vec3::new(5.0, 1.0, 0.0);
        let ray_direction = Vec3::new(0.0, -1.0, 0.0);

        assert!(plane.ray_intersect(&ray_origin, &ray_direction).is_none());
    }

    #[test]
    fn test_plane_miss_opposite_direction() {
        let material = Material::new(Color::new(150, 150, 150));
        let plane = Plane::new(-1.0, 3.0, material);
        let ray_origin = Vec3::new(0.0, 1.0, 0.0);
        let ray_direction = Vec3::new(0.0, 1.0, 0.0);

        assert!(plane.ray_intersect(&ray_origin, &ray_direction).is_none());
    }
}

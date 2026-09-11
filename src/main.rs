mod camera;
mod color;
mod cube;
mod framebuffer;
mod light;
mod plane;
mod ray_intersect;
mod texture;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{dot, normalize, Vec3};
use std::f32::consts::PI;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use crate::camera::Camera;
use crate::color::Color;
use crate::cube::Cube;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::plane::Plane;
use crate::ray_intersect::{Intersect, Material, RayIntersect};
use crate::texture::Texture;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// Color de fondo azul oscuro
const BACKGROUND_COLOR: u32 = 0x0A1528;

const FOV: f32 = PI / 3.0;
const ROTATION_SPEED: f32 = PI / 60.0;
const SHADOW_BIAS: f32 = 1e-3;

pub fn cast_shadow(
    intersect: &Intersect,
    light_direction: &Vec3,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> bool {
    let shadow_ray_origin = intersect.point + intersect.normal * SHADOW_BIAS;
    let light_distance = (light.position - intersect.point).magnitude();

    objects.iter().any(|object| {
        object
            .ray_intersect(&shadow_ray_origin, light_direction)
            .as_ref()
            .is_some_and(|blocker| blocker.distance < light_distance)
    })
}

pub fn shade(
    intersect: &Intersect,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> Color {
    let light_direction = (light.position - intersect.point).normalize();

    let light_intensity = if cast_shadow(intersect, &light_direction, light, objects) {
        0.0
    } else {
        light.intensity
    };

    // Sombreado difuso de Lambert
    let ambient_intensity = 0.15;
    let diffuse_intensity = dot(&intersect.normal, &light_direction).max(0.0);
    let total_intensity = (ambient_intensity + diffuse_intensity * light_intensity).min(1.0);

    // Muestrear el color difuso de la textura según las coordenadas UV
    let diffuse_color = intersect.material.get_diffuse_color(intersect.u, intersect.v);
    diffuse_color * total_intensity
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Box<dyn RayIntersect>],
    light: &Light,
) -> Color {
    let mut closest: Option<Intersect> = None;

    for object in objects {
        if let Some(intersect) = object.ray_intersect(ray_origin, ray_direction) {
            if closest
                .as_ref()
                .is_none_or(|current| intersect.distance < current.distance)
            {
                closest = Some(intersect);
            }
        }
    }

    let Some(intersect) = closest else {
        return Color::from_hex(BACKGROUND_COLOR);
    };

    shade(&intersect, light, objects)
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Box<dyn RayIntersect>],
    camera: &Camera,
    light: &Light,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    let perspective_scale = (FOV / 2.0).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let ray_direction = camera.basis_change(&ray_direction);

            framebuffer.set_current_color(
                cast_ray(&camera.eye, &ray_direction, objects, light).to_hex(),
            );
            framebuffer.point(x, y);
        }
    }
}

fn create_textured_cube() -> Cube {
    let center = Vec3::new(0.0, 0.0, 0.0);
    let size = 1.5;

    // Caso 1: Texturas separadas para arriba/abajo y laterales (sides.png y up-down.png)
    let up_down_path = "assets/up-down.png";
    let sides_path = "assets/sides.png";

    if Path::new(up_down_path).exists() && Path::new(sides_path).exists() {
        let top_bottom = Texture::from_file(up_down_path);
        let sides = Texture::from_file(sides_path);

        if let (Ok(tb_tex), Ok(s_tex)) = (top_bottom, sides) {
            println!("[Textura] Cargadas texturas de caras compuestas:");
            println!(" - Arriba / Abajo (+Y, -Y): {}", up_down_path);
            println!(" - Laterales (+X, -X, +Z, -Z): {}", sides_path);
            return Cube::with_face_materials(
                center,
                size,
                Material::with_texture(Arc::new(tb_tex)),
                Material::with_texture(Arc::new(s_tex)),
            );
        }
    }

    // Caso 2: Una sola imagen para las 6 caras
    let possible_single = [
        "assets/cube.png",
        "assets/sides.png",
        "assets/texture.png",
        "assets/textura.png",
    ];

    for path in possible_single {
        if Path::new(path).exists() {
            if let Ok(tex) = Texture::from_file(path) {
                println!("[Textura] Cargada textura unica para las 6 caras desde: {}", path);
                return Cube::new(center, size, Material::with_texture(Arc::new(tex)));
            }
        }
    }

    // Caso 3: Textura procedural de prueba si no hay imágenes
    println!("[Textura] Usando textura procedural de prueba en verde aqua.");
    Cube::new(center, size, Material::with_texture(Arc::new(Texture::default_grid())))
}

fn main() {
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Raytracer - Cubo 3D Texturizado",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Crear el cubo con sus texturas cargadas
    let cube = create_textured_cube();

    // Piso acotado de color físico neutro mate
    let floor_material = Material::new(Color::new(145, 150, 160));
    let floor = Plane::new(-0.75, 2.5, floor_material);

    let objects: Vec<Box<dyn RayIntersect>> = vec![
        Box::new(cube),
        Box::new(floor),
    ];

    // Luz puntual para generar iluminación difusa y proyectar sombras
    let light = Light::new(Vec3::new(4.0, 5.0, 6.0), Color::new(255, 255, 255), 0.85);

    // Cámara orbital apuntando al centro del cubo (0, 0, 0)
    let mut camera = Camera::new(
        Vec3::new(2.2, 1.8, 3.2),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut camera_moved = true;

    println!("=== Raytracer Cubo 3D Texturizado ===");
    println!("Controles de camara orbital:");
    println!(" - Flechas o WASD: Rotar la camara orbitalmente alrededor del cubo");
    println!(" - Escape: Salir");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let orbit = [
            (Key::Left, ROTATION_SPEED, 0.0),
            (Key::Right, -ROTATION_SPEED, 0.0),
            (Key::Up, 0.0, -ROTATION_SPEED),
            (Key::Down, 0.0, ROTATION_SPEED),
            (Key::A, ROTATION_SPEED, 0.0),
            (Key::D, -ROTATION_SPEED, 0.0),
            (Key::W, 0.0, -ROTATION_SPEED),
            (Key::S, 0.0, ROTATION_SPEED),
        ];

        for (key, delta_yaw, delta_pitch) in orbit {
            if window.is_key_down(key) {
                camera.orbit(delta_yaw, delta_pitch);
                camera_moved = true;
            }
        }

        if camera_moved {
            render(&mut framebuffer, &objects, &camera, &light);
            camera_moved = false;
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

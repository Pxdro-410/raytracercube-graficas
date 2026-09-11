# Raytracer 3D - Cubo Texturizado con Luz Difusa y Camara Orbital

Raytracer interactivo desarrollado en Rust enfocado en el renderizado de un cubo tridimensional texturizado mediante mapeo UV, con soporte para texturas compuestas (caras laterales y base/tapa), modelo de iluminacion difusa pura (Lambert), proyeccion de sombras duras y navegacion interactiva en tiempo real mediante una camara orbital.

### Pedro Caso - 241286

---

## Demostracion



---

## Caracteristicas Principales

- **Mapeo UV en el Cubo**: Calculo analitico de coordenadas de textura en cada una de las 6 caras del cubo segun la normal de impacto.
- **Soporte de Texturas Compuestas**: Capacidad de aplicar texturas diferenciadas para las caras laterales (`sides.png`) y las caras superior e inferior (`up-down.png`), asi como texturas unicas globales o procedurales de respaldo.
- **Decodificacion de Imagenes**: Integracion de la biblioteca `image` para cargar archivos PNG y JPEG desde el directorio `assets/`.
- **Sombreado Difuso con Texturas**: El color muestreado de la textura se modula en tiempo real con el modelo difuso de Lambert ($N \cdot L$) y una base ambiental.
- **Proyeccion de Sombras en Tiempo Real**: Lanzamiento de rayos de sombra (*Shadow Rays*) hacia la fuente de luz puntual para proyectar sombras directas sobre el piso.
- **Piso Acotado**: Plataforma horizontal con limites en $X$ y $Z$ con material neutro mate que sirve de base bajo el cubo.
- **Camara Orbital Interactiva**: Navegacion orbital en coordenadas esfericas (yaw y pitch) con limites angulares para evitar bloqueo de cardan (*gimbal lock*).
- **Ventana y Framebuffer**: Despliegue interactivo mediante la biblioteca `minifb` a una resolucion de 800x600 pixeles.

---

## Estructura del Proyecto

```text
raytracercube-graficas/
├── assets/
│   ├── sides.png          # Textura para las 4 caras laterales (+X, -X, +Z, -Z)
│   └── up-down.png        # Textura para las caras superior e inferior (+Y, -Y)
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src/
    ├── main.rs            # Punto de entrada, deteccion de texturas y ciclo de renderizado
    ├── camera.rs          # Camara orbital con cambio de base y control por teclado
    ├── cube.rs            # Primitiva de cubo con mapeo UV y soporte de texturas por cara
    ├── plane.rs           # Primitiva de piso horizontal acotado
    ├── texture.rs         # Carga de imagenes, decodificacion y muestreo UV
    ├── light.rs           # Definicion de la fuente de luz puntual
    ├── ray_intersect.rs   # Trait RayIntersect, Intersect con UVs y Material con textura
    ├── color.rs           # Estructura Color RGB con sobrecarga de operadores
    └── framebuffer.rs     # Buffer lineal de pixeles para renderizado
```

### Descripcion de Modulos y Recursos

- **`assets/`**: Carpeta que almacena las imagenes de textura (`sides.png` y `up-down.png`).
- **`src/main.rs`**: Gestiona el ciclo de eventos de la ventana, detecta y carga automaticamente las texturas de `assets/`, coordina los controles de usuario y ejecuta el algoritmo de trazado de rayos.
- **`src/texture.rs`**: Decodifica archivos de imagen en memoria con el crate `image` y provee el metodo `get_color(u, v)` con soporte de envoltura (*wrap*) para obtener el color del texel correspondiente.
- **`src/cube.rs`**: Implementa la interseccion rayo-AABB (*Slab Method*), calcula las coordenadas normalizadas $(u, v)$ de cada cara y aplica el material correspondiente (lados o superior/inferior).
- **`src/camera.rs`**: Controla la orientacion y posicion de la camara, implementando la transformacion de base y la rotacion orbital alrededor del cubo.
- **`src/plane.rs`**: Modela el piso acotado donde se proyecta la sombra difusa del cubo.
- **`src/ray_intersect.rs`**: Define la estructura `Intersect` (incluyendo coordenadas $u$ y $v$), el trait `RayIntersect` y la estructura `Material` compatible con texturas en memoria.
- **`src/light.rs`**: Representa la fuente de iluminacion puntual con posicion, color e intensidad.
- **`src/color.rs`**: Define la estructura de color de 24 bits y sus operaciones matematicas basicas.
- **`src/framebuffer.rs`**: Almacena y actualiza el buffer de dibujo en pantalla.

---

## Gestion de Texturas

El programa detecta y carga automaticamente las texturas colocadas en la carpeta `assets/`:

1. **Texturas compuestas**:
   - `assets/up-down.png`: Se mapea en la cara superior ($+Y$) e inferior ($-Y$).
   - `assets/sides.png`: Se mapea en las cuatro caras laterales ($+X$, $-X$, $+Z$, $-Z$).
2. **Textura unica**: Si solo existe un archivo nombrado `assets/cube.png` o `assets/texture.png`, este se aplica de manera uniforme a las 6 caras.
3. **Respaldo procedural**: Si la carpeta `assets/` no contiene imagenes, el programa genera una textura cuadriculada para validar el mapeo UV sin interrumpir la ejecucion.

---

## Requisitos Previos

- **Rust**: Version 1.70 o superior (con `cargo` incluido).
- Sistema Operativo: Windows, macOS o Linux.

---

## Compilacion y Ejecucion

### 1. Ejecutar Pruebas Unitarias
Para validar las formulas de interseccion de rayos, calculo de UVs y muestreo de texturas:
```bash
cargo test
```

### 2. Ejecutar en Modo Desarrollo
```bash
cargo run
```

### 3. Ejecutar con Optimizaciones (Modo Release)
Recomendado para una tasa de refresco optima durante la rotacion orbital con texturas:
```bash
cargo run --release
```

---

## Controles de Usuario

Una vez iniciada la aplicacion interactiva:

| Tecla | Accion |
| :--- | :--- |
| Flecha Izquierda / A | Rotar camara hacia la izquierda (Yaw positivo) |
| Flecha Derecha / D | Rotar camara hacia la derecha (Yaw negativo) |
| Flecha Arriba / W | Rotar camara hacia arriba (Pitch negativo) |
| Flecha Abajo / S | Rotar camara hacia abajo (Pitch positivo) |
| Escape | Cerrar la aplicacion |

---

## Dependencias

- **`nalgebra-glm`**: Algebra lineal para calculos vectoriales, normales y transformaciones.
- **`minifb`**: Administracion de ventana nativa e intercambio del buffer de pixeles.
- **`image`**: Lectura y decodificacion de formatos de imagen rasterizada (PNG, JPEG).

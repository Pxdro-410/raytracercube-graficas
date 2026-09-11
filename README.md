# Raytracer 3D - Cubo con Luz Difusa y Camara Orbital

Raytracer desarrollado en Rust enfocado en el renderizado de un cubo tridimensional con modelo de iluminacion difusa pura (Lambert), proyeccion de sombras duras y navegacion interactiva en tiempo real mediante una camara orbital.

### Pedro Caso - 241286

---

## Demostracion

<img width="990" height="744" alt="ray_tracer_cubo-PC" src="https://github.com/user-attachments/assets/fdedc7ef-955f-459d-9fe5-60f840e77ebd" />


---

## Caracteristicas Principales

- **Interseccion Rayo-Cubo (AABB)**: Implementacion del algoritmo de corte de planos delimitadores (*Slab Method*) para calcular colisiones precisas y determinar la normal analitica de cada cara impactada.
- **Sombreado Difuso Puro**: Implementacion del modelo de reflexion difusa de Lambert ($N \cdot L$), combinado con una iluminacion ambiental minima para preservar el volumen tridimensional del objeto.
- **Proyeccion de Sombras en Tiempo Real**: Lanzamiento de rayos de sombra (*Shadow Rays*) hacia la fuente de luz puntual para proyectar sombras directas sobre el piso.
- **Piso Acotado**: Plataforma horizontal con limites en $X$ y $Z$ que actua como escenario fisico bajo el cubo.
- **Camara Orbital Interactiva**: Sistema de camara centrado en el origen con conversion a coordenadas esfericas (yaw y pitch) con limites angulares para evitar bloqueo de cardan (*gimbal lock*).
- **Ventana y Framebuffer**: Despliegue interactivo mediante la biblioteca `minifb` a una resolucion de 800x600 pixeles.

---

## Estructura del Proyecto

```text
raytracercube-graficas/
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src/
    ├── main.rs            # Punto de entrada, bucle interactivo y sombreado
    ├── camera.rs          # Camara con cambio de base y rotacion orbital
    ├── cube.rs            # Primitiva de cubo e interseccion AABB
    ├── plane.rs           # Primitiva de piso horizontal acotado
    ├── light.rs           # Definicion de fuente de luz puntual
    ├── ray_intersect.rs   # Trait RayIntersect, Intersect y Material
    ├── color.rs           # Estructura Color RGB con sobrecarga de operadores
    └── framebuffer.rs     # Buffer lineal de pixeles para renderizado
```

### Descripcion de Modulos

- **`main.rs`**: Gestiona el ciclo principal de eventos de la ventana, procesa las entradas de teclado para rotar la camara y coordina las funciones `render()`, `cast_ray()` y `shade()`.
- **`camera.rs`**: Define la posicion de la camara (`eye`), el objetivo (`center`) y el vector de orientacion (`up`). Implementa la funcion `orbit()` para modificar la posicion orbital sobre una esfera virtual alrededor del objetivo.
- **`cube.rs`**: Define el cubo mediante sus limites minimos y maximos (`min` y `max`). Utiliza el algoritmo de slabs para calcular los tiempos de entrada y salida del rayo en cada dimension y extrae la normal unitaria correspondiente a la cara atravesada.
- **`plane.rs`**: Representa una superficie plana horizontal en una coordenada $Y$ fija con un tamano maximo (`half_size`), permitiendo delimitar el area visible del suelo.
- **`light.rs`**: Modela una fuente de iluminacion puntual con posicion espacial, color e intensidad.
- **`ray_intersect.rs`**: Define el trait comun `RayIntersect` implementado por las figuras, asi como las estructuras `Intersect` (punto de impacto, normal, distancia) y `Material`.
- **`color.rs`**: Proporciona operaciones aritmeticas para colores (suma saturada, multiplicacion escalar) y conversion a formato entero hexadecimal de 32 bits (0xRRGGBB).
- **`framebuffer.rs`**: Almacena el arreglo de pixeles en memoria y provee el metodo de dibujado punto por punto.

---

## Requisitos Previos

- **Rust**: Version 1.70 o superior (con `cargo` incluido).
- Sistema Operativo: Windows, macOS o Linux.

---

## Compilacion y Ejecucion

### 1. Clonar el Repositorio
```bash
git clone https://github.com/Pxdro-410/raytracercube-graficas.git
cd raytracercube-graficas
```

### 2. Ejecutar Pruebas Unitarias
El proyecto cuenta con pruebas automatizadas para verificar las formulas de interseccion de rayos:
```bash
cargo test
```

### 3. Ejecutar en Modo Desarrollo
```bash
cargo run
```

### 4. Ejecutar con Optimizaciones (Modo Release)
Para obtener la maxima tasa de refresco y rendimiento durante la rotacion:
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

- **`nalgebra-glm`**: Operaciones de algebra lineal para vectores tridimensionales, productos punto y cruz, y normalizaciones.
- **`minifb`**: Creacion de ventana nativa de bajo nivel e intercambio de buffer de pixeles.

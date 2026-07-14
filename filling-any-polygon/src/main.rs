// Lab 1 - Relleno de polígonos
// Usamos raylib solo para tener un "framebuffer" (una Image en memoria)
// y para exportar la imagen final. El dibujo lo hacemos nosotros:
//   - Líneas con el algoritmo de Bresenham.
//   - Relleno con scanline (regla par-impar), que sirve para polígonos
//     cóncavos, de muchos puntos, e incluso con agujeros.

use raylib::prelude::*;

// Tamaño del framebuffer. Los puntos máximos son x=761, y=410,
// así que con 800x460 caben todos con un pequeño margen.
const WIDTH: i32 = 800;
const HEIGHT: i32 = 460;

// ---------------------------------------------------------------------------
// Framebuffer: envuelve una Image de raylib y nos deja pintar pixel a pixel.
// ---------------------------------------------------------------------------
struct Framebuffer {
    image: Image,
}

impl Framebuffer {
    fn new(width: i32, height: i32, background: Color) -> Framebuffer {
        Framebuffer {
            image: Image::gen_image_color(width, height, background),
        }
    }

    // Pinta un pixel, ignorando los que se salen de la imagen.
    fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT {
            self.image.draw_pixel(x, y, color);
        }
    }

    // Guarda el framebuffer a un archivo (el formato lo decide la extensión).
    fn export(&self, path: &str) {
        self.image.export_image(path);
    }
}

// ---------------------------------------------------------------------------
// Bresenham: dibuja una línea recta entre dos puntos usando solo enteros.
// ---------------------------------------------------------------------------
fn draw_line(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0;
    let mut y = y0;
    loop {
        fb.set_pixel(x, y, color);
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

// ---------------------------------------------------------------------------
// Relleno por scanline con regla par-impar.
// Recibe una lista de "contornos" (cada uno es un polígono). El primero es el
// borde exterior y los siguientes son agujeros: con par-impar los agujeros
// quedan sin pintar automáticamente.
// ---------------------------------------------------------------------------
fn fill_polygon(fb: &mut Framebuffer, contours: &[Vec<(i32, i32)>], color: Color) {
    // Buscamos el rango vertical (min y, max y) de todos los contornos.
    let mut min_y = HEIGHT;
    let mut max_y = 0;
    for contour in contours {
        for &(_, y) in contour {
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }

    // Recorremos cada línea horizontal (scanline).
    for y in min_y..=max_y {
        let yc = y as f32 + 0.5; // muestreamos en el centro del pixel
        let mut crossings: Vec<f32> = Vec::new();

        // Buscamos en qué x cruza la scanline cada arista de cada contorno.
        for contour in contours {
            let n = contour.len();
            for i in 0..n {
                let (x0, y0) = contour[i];
                let (x1, y1) = contour[(i + 1) % n]; // arista i -> i+1 (cierra el polígono)
                let (fx0, fy0) = (x0 as f32, y0 as f32);
                let (fx1, fy1) = (x1 as f32, y1 as f32);

                // La scanline cruza la arista si yc está entre y0 y y1.
                if (fy0 <= yc && yc < fy1) || (fy1 <= yc && yc < fy0) {
                    let x = fx0 + (yc - fy0) / (fy1 - fy0) * (fx1 - fx0);
                    crossings.push(x);
                }
            }
        }

        // Ordenamos los cruces y pintamos entre pares: (0,1), (2,3), ...
        crossings.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut i = 0;
        while i + 1 < crossings.len() {
            let x_start = crossings[i].ceil() as i32;
            let x_end = crossings[i + 1].floor() as i32;
            for x in x_start..=x_end {
                fb.set_pixel(x, y, color);
            }
            i += 2;
        }
    }
}

// Dibuja el contorno de un polígono (línea entre punto y punto, cerrándolo).
fn draw_outline(fb: &mut Framebuffer, contour: &[(i32, i32)], color: Color) {
    let n = contour.len();
    for i in 0..n {
        let (x0, y0) = contour[i];
        let (x1, y1) = contour[(i + 1) % n];
        draw_line(fb, x0, y0, x1, y1, color);
    }
}

fn main() {
    let mut fb = Framebuffer::new(WIDTH, HEIGHT, Color::new(0, 0, 0, 255)); // fondo negro
    let line_color = Color::new(255, 255, 255, 255); // borde blanco para todos

    // ---- Definición de los polígonos ----
    let poly1: Vec<(i32, i32)> = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];
    let poly2: Vec<(i32, i32)> = vec![
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];
    let poly3: Vec<(i32, i32)> = vec![
        (377, 249), (411, 197), (436, 249),
    ];
    let poly4: Vec<(i32, i32)> = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37),
        (660, 52), (750, 145), (761, 179), (672, 192), (659, 214), (615, 214),
        (632, 230), (580, 230), (597, 215), (552, 214), (517, 144), (466, 180),
    ];
    // Polígono 5: es un agujero dentro del polígono 4, NO se pinta.
    let poly5: Vec<(i32, i32)> = vec![
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];

    // ---- Relleno (scanline) ----
    fill_polygon(&mut fb, &[poly1.clone()], Color::new(230, 70, 70, 255));   // rojo
    fill_polygon(&mut fb, &[poly2.clone()], Color::new(70, 200, 100, 255));  // verde
    fill_polygon(&mut fb, &[poly3.clone()], Color::new(80, 130, 235, 255));  // azul
    // Polígono 4 con su agujero (poly5): par-impar deja el agujero sin pintar.
    fill_polygon(&mut fb, &[poly4.clone(), poly5.clone()], Color::new(240, 200, 50, 255)); // amarillo

    // ---- Contornos (Bresenham) encima del relleno ----
    draw_outline(&mut fb, &poly1, line_color);
    draw_outline(&mut fb, &poly2, line_color);
    draw_outline(&mut fb, &poly3, line_color);
    draw_outline(&mut fb, &poly4, line_color);
    draw_outline(&mut fb, &poly5, line_color); // borde del agujero

    // ---- Exportar ----
    fb.export("out.png");
    fb.export("out.bmp");
    println!("Listo: se generaron out.png y out.bmp");
}

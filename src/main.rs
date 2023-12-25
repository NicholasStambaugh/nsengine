use minifb::{Key, Window, WindowOptions};
use std::io::{self, Write};

#[derive(Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Pixel>>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        let pixels = vec![vec![Pixel { r: 0, g: 0, b: 0 }; width]; height];
        Canvas { width, height, pixels }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Pixel) {
        if x < self.width && y < self.height {
            self.pixels[y][x] = color;
        }
    }

    fn render(&self, buffer: &mut Vec<u32>) {
        // Render the canvas to the buffer
        for row in &self.pixels {
            for pixel in row {
                let color = (pixel.r as u32) << 16 | (pixel.g as u32) << 8 | pixel.b as u32;
                buffer.push(color);
            }
        }
    }
}

fn main() {
    let width = 40;
    let height = 20;

    let mut canvas = Canvas::new(width, height);
    let mut buffer: Vec<u32> = Vec::new();

    // Draw a simple red rectangle
    for x in 5..35 {
        for y in 5..35 {
            canvas.set_pixel(x, y, Pixel { r: 255, g: 0, b: 0 });
        }
    }

    // Draw a green line
    for x in 5..35 {
        canvas.set_pixel(x, 10, Pixel { r: 0, g: 255, b: 0 });
    }

    // Set up the window
    let mut window = Window::new(
        "Rust 2D Graphics",
        width,
        height,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.clear();
        canvas.render(&mut buffer);
        window
            .update_with_buffer(&buffer, width, height)
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
    }
}

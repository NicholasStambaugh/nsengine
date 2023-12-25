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
    let width = 1000;
    let height = 1080;

    let mut canvas = Canvas::new(width, height);
    let mut buffer: Vec<u32> = Vec::new();

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

    let mut cursor_x = width / 2;
    let mut cursor_y = height / 2;

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle user input
        if window.is_key_down(Key::Right) && cursor_x < width - 1 {
            cursor_x += 1;
        }
        if window.is_key_down(Key::Left) && cursor_x > 0 {
            cursor_x -= 1;
        }
        if window.is_key_down(Key::Down) && cursor_y < height - 1 {
            cursor_y += 1;
        }
        if window.is_key_down(Key::Up) && cursor_y > 0 {
            cursor_y -= 1;
        }

        // Draw on the canvas based on user input
        canvas.set_pixel(cursor_x, cursor_y, Pixel { r: 255, g: 255, b: 255 });

        // Render and update the window
        buffer.clear();
        canvas.render(&mut buffer);
        window
            .update_with_buffer(&buffer, width, height)
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
    }
}

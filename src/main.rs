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

    fn render(&self) {
        // Render the canvas to the console
        for row in &self.pixels {
            for pixel in row {
                print!(
                    "\x1b[38;2;{};{};{}m█\x1b[0m",
                    pixel.r, pixel.g, pixel.b
                );
            }
            println!(); // Move to the next row
        }
    }
}

fn main() {
    let mut canvas = Canvas::new(40, 20);

    // Draw a simple red rectangle
    for x in 5..15 {
        for y in 5..15 {
            canvas.set_pixel(x, y, Pixel { r: 255, g: 0, b: 0 });
        }
    }

    // Draw a green line
    for x in 5..35 {
        canvas.set_pixel(x, 10, Pixel { r: 0, g: 255, b: 0 });
    }

    // Render the canvas
    canvas.render();

    // Wait for user input before exiting
    let mut input = String::new();
    print!("Press Enter to exit...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
}

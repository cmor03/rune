//! Fixed-size grayscale canvas and drawing primitives.

use crate::font;

/// Rune display width in pixels.
pub const WIDTH: usize = 480;
/// Rune display height in pixels.
pub const HEIGHT: usize = 280;

/// A grayscale color.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color(pub u8);

/// Integer rectangle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rect {
    /// Left coordinate.
    pub x: i32,
    /// Top coordinate.
    pub y: i32,
    /// Width in pixels.
    pub w: i32,
    /// Height in pixels.
    pub h: i32,
}

impl Rect {
    /// Creates a rectangle.
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    /// Returns an inset rectangle.
    pub const fn inset(self, amount: i32) -> Self {
        Self {
            x: self.x + amount,
            y: self.y + amount,
            w: self.w - amount * 2,
            h: self.h - amount * 2,
        }
    }
}

/// A fixed Rune framebuffer.
pub struct Canvas {
    pixels: Vec<u8>,
}

impl Canvas {
    /// Creates a canvas initialized to `color`.
    pub fn new(color: Color) -> Self {
        Self {
            pixels: vec![color.0; WIDTH * HEIGHT],
        }
    }

    /// Clears the canvas.
    pub fn clear(&mut self, color: Color) {
        self.pixels.fill(color.0);
    }

    /// Returns the raw grayscale pixels in row-major order.
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    /// Draws one pixel if it is in bounds.
    pub fn pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= WIDTH || y >= HEIGHT {
            return;
        }
        self.pixels[y * WIDTH + x] = color.0;
    }

    /// Draws a filled rectangle.
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        let x0 = rect.x.max(0) as usize;
        let y0 = rect.y.max(0) as usize;
        let x1 = (rect.x + rect.w).min(WIDTH as i32).max(0) as usize;
        let y1 = (rect.y + rect.h).min(HEIGHT as i32).max(0) as usize;
        for y in y0..y1 {
            let row = y * WIDTH;
            for x in x0..x1 {
                self.pixels[row + x] = color.0;
            }
        }
    }

    /// Draws a rectangle outline.
    pub fn stroke_rect(&mut self, rect: Rect, color: Color) {
        self.line(rect.x, rect.y, rect.x + rect.w - 1, rect.y, color);
        self.line(rect.x, rect.y, rect.x, rect.y + rect.h - 1, color);
        self.line(
            rect.x + rect.w - 1,
            rect.y,
            rect.x + rect.w - 1,
            rect.y + rect.h - 1,
            color,
        );
        self.line(
            rect.x,
            rect.y + rect.h - 1,
            rect.x + rect.w - 1,
            rect.y + rect.h - 1,
            color,
        );
    }

    /// Draws a simple rounded panel using square pixels and clipped corners.
    pub fn panel(&mut self, rect: Rect, fill: Color, stroke: Color) {
        self.fill_rect(Rect::new(rect.x + 3, rect.y, rect.w - 6, rect.h), fill);
        self.fill_rect(Rect::new(rect.x, rect.y + 3, rect.w, rect.h - 6), fill);
        self.stroke_rect(Rect::new(rect.x + 3, rect.y, rect.w - 6, rect.h), stroke);
        self.stroke_rect(Rect::new(rect.x, rect.y + 3, rect.w, rect.h - 6), stroke);
    }

    /// Draws a line using Bresenham rasterization.
    pub fn line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Color) {
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.pixel(x0, y0, color);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    /// Draws a filled circle.
    pub fn fill_circle(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        let r2 = radius * radius;
        for y in -radius..=radius {
            for x in -radius..=radius {
                if x * x + y * y <= r2 {
                    self.pixel(cx + x, cy + y, color);
                }
            }
        }
    }

    /// Draws text using the built-in 5x7 uppercase bitmap font.
    pub fn text(&mut self, x: i32, y: i32, text: &str, scale: i32, color: Color) {
        let mut cursor = x;
        let step = 6 * scale;
        for ch in text.chars() {
            if ch == '\n' {
                cursor = x;
                continue;
            }
            font::draw_char(self, cursor, y, ch, scale, color);
            cursor += step;
        }
    }

    /// Draws text centered around `cx`.
    pub fn text_center(&mut self, cx: i32, y: i32, text: &str, scale: i32, color: Color) {
        let width = text.chars().count() as i32 * 6 * scale - scale;
        self.text(cx - width / 2, y, text, scale, color);
    }

    /// Applies a checker/dot texture similar to a low-power LCD/e-ink surface.
    pub fn surface_texture(&mut self, amount: u8) {
        for y in (0..HEIGHT).step_by(4) {
            for x in (0..WIDTH).step_by(4) {
                let idx = y * WIDTH + x;
                self.pixels[idx] = self.pixels[idx].saturating_sub(amount);
            }
        }
    }
}

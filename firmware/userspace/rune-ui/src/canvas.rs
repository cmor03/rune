//! Fixed-size grayscale canvas and drawing primitives.

use crate::font::{self, Face};

/// Prototype portrait LCD width in pixels.
pub const WIDTH: usize = 320;
/// Prototype portrait LCD height in pixels.
pub const HEIGHT: usize = 480;

const QUAD_TOP: u8 = 0b0001;
const QUAD_RIGHT: u8 = 0b0010;
const QUAD_BOTTOM: u8 = 0b0100;
const QUAD_LEFT: u8 = 0b1000;
const QUAD_ALL: u8 = QUAD_TOP | QUAD_RIGHT | QUAD_BOTTOM | QUAD_LEFT;

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

fn has_quadrants(mask: u8, required: u8) -> bool {
    mask & required == required
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

    /// Blends one antialiased pixel over the current framebuffer.
    pub fn blend_pixel(&mut self, x: i32, y: i32, color: Color, coverage: f32) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= WIDTH || y >= HEIGHT {
            return;
        }
        let coverage = coverage.clamp(0.0, 1.0);
        let idx = y * WIDTH + x;
        let bg = self.pixels[idx] as f32;
        let fg = color.0 as f32;
        self.pixels[idx] = (bg + (fg - bg) * coverage).round().clamp(0.0, 255.0) as u8;
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
        self.round_panel(rect, 5, fill, stroke);
    }

    /// Draws a filled rounded rectangle.
    pub fn fill_round_rect(&mut self, rect: Rect, radius: i32, color: Color) {
        let radius = radius.max(0).min(rect.w / 2).min(rect.h / 2);
        if radius == 0 {
            self.fill_rect(rect, color);
            return;
        }
        self.fill_rect(Rect::new(rect.x + radius, rect.y, rect.w - radius * 2, rect.h), color);
        self.fill_rect(Rect::new(rect.x, rect.y + radius, rect.w, rect.h - radius * 2), color);
        self.fill_circle(rect.x + radius, rect.y + radius, radius, color);
        self.fill_circle(rect.x + rect.w - radius - 1, rect.y + radius, radius, color);
        self.fill_circle(rect.x + radius, rect.y + rect.h - radius - 1, radius, color);
        self.fill_circle(
            rect.x + rect.w - radius - 1,
            rect.y + rect.h - radius - 1,
            radius,
            color,
        );
    }

    /// Draws a rounded rectangle outline.
    pub fn stroke_round_rect(&mut self, rect: Rect, radius: i32, color: Color) {
        let radius = radius.max(0).min(rect.w / 2).min(rect.h / 2);
        if radius == 0 {
            self.stroke_rect(rect, color);
            return;
        }
        let left = rect.x;
        let right = rect.x + rect.w - 1;
        let top = rect.y;
        let bottom = rect.y + rect.h - 1;

        self.line(left + radius, top, right - radius, top, color);
        self.line(left + radius, bottom, right - radius, bottom, color);
        self.line(left, top + radius, left, bottom - radius, color);
        self.line(right, top + radius, right, bottom - radius, color);
        self.stroke_circle_quadrants(
            left + radius,
            top + radius,
            radius,
            color,
            QUAD_TOP | QUAD_LEFT,
        );
        self.stroke_circle_quadrants(
            right - radius,
            top + radius,
            radius,
            color,
            QUAD_TOP | QUAD_RIGHT,
        );
        self.stroke_circle_quadrants(
            left + radius,
            bottom - radius,
            radius,
            color,
            QUAD_BOTTOM | QUAD_LEFT,
        );
        self.stroke_circle_quadrants(
            right - radius,
            bottom - radius,
            radius,
            color,
            QUAD_BOTTOM | QUAD_RIGHT,
        );
    }

    /// Draws a filled rounded rectangle with a rounded outline.
    pub fn round_panel(&mut self, rect: Rect, radius: i32, fill: Color, stroke: Color) {
        self.fill_round_rect(rect, radius, fill);
        self.stroke_round_rect(rect, radius, stroke);
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

    /// Draws a circle outline.
    pub fn stroke_circle(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        self.stroke_circle_quadrants(cx, cy, radius, color, QUAD_ALL);
    }

    /// Draws text using an embedded outline font.
    pub fn text(&mut self, x: i32, y: i32, text: &str, face: Face, size: f32, color: Color) {
        self.text_tracked(x, y, text, face, size, 0.0, color);
    }

    /// Draws text using an embedded outline font and extra letter spacing.
    pub fn text_tracked(
        &mut self,
        x: i32,
        y: i32,
        text: &str,
        face: Face,
        size: f32,
        tracking: f32,
        color: Color,
    ) {
        font::draw(
            self,
            x,
            y,
            text,
            font::TextStyle::new(face, size, tracking, color),
        );
    }

    /// Draws text centered around `cx`.
    pub fn text_center(&mut self, cx: i32, y: i32, text: &str, face: Face, size: f32, color: Color) {
        self.text_center_tracked(cx, y, text, face, size, 0.0, color);
    }

    /// Draws tracked text centered around `cx`.
    pub fn text_center_tracked(
        &mut self,
        cx: i32,
        y: i32,
        text: &str,
        face: Face,
        size: f32,
        tracking: f32,
        color: Color,
    ) {
        let width = font::measure(face, text, size, tracking);
        self.text_tracked(
            (cx as f32 - width / 2.0).round() as i32,
            y,
            text,
            face,
            size,
            tracking,
            color,
        );
    }

    /// Draws text with its right edge at `right`.
    pub fn text_right(
        &mut self,
        right: i32,
        y: i32,
        text: &str,
        face: Face,
        size: f32,
        color: Color,
    ) {
        let width = font::measure(face, text, size, 0.0);
        self.text((right as f32 - width).round() as i32, y, text, face, size, color);
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

    fn stroke_circle_quadrants(
        &mut self,
        cx: i32,
        cy: i32,
        radius: i32,
        color: Color,
        quadrants: u8,
    ) {
        let mut x = radius;
        let mut y = 0;
        let mut err = 0;
        while x >= y {
            if has_quadrants(quadrants, QUAD_TOP | QUAD_RIGHT) {
                self.pixel(cx + x, cy - y, color);
                self.pixel(cx + y, cy - x, color);
            }
            if has_quadrants(quadrants, QUAD_BOTTOM | QUAD_RIGHT) {
                self.pixel(cx + x, cy + y, color);
                self.pixel(cx + y, cy + x, color);
            }
            if has_quadrants(quadrants, QUAD_BOTTOM | QUAD_LEFT) {
                self.pixel(cx - x, cy + y, color);
                self.pixel(cx - y, cy + x, color);
            }
            if has_quadrants(quadrants, QUAD_TOP | QUAD_LEFT) {
                self.pixel(cx - x, cy - y, color);
                self.pixel(cx - y, cy - x, color);
            }
            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }
}

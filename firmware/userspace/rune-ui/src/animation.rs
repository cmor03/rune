//! Animation timing helpers and pixel sprite data.

use crate::canvas::{Canvas, Color, Rect, HEIGHT, WIDTH};

/// Render-time animation context.
#[derive(Clone, Copy, Debug)]
pub struct FrameContext {
    /// Current frame index.
    pub frame: u32,
    /// Total frames requested by the caller.
    pub total: u32,
    /// Battery percentage.
    pub battery_pct: u8,
    /// Current launcher selection.
    pub selected: usize,
}

impl FrameContext {
    /// Returns a frame context with stable defaults.
    pub const fn new(frame: u32, total: u32) -> Self {
        Self {
            frame,
            total,
            battery_pct: 72,
            selected: 0,
        }
    }

    /// Returns a copy with a launcher selection.
    pub const fn with_selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }

    /// Returns a triangle wave in the range 0..=amplitude.
    pub fn pulse(self, period: u32, amplitude: i32) -> i32 {
        let p = period.max(2);
        let t = (self.frame % p) as i32;
        let half = (p / 2) as i32;
        if t <= half {
            t * amplitude / half.max(1)
        } else {
            (p as i32 - t) * amplitude / half.max(1)
        }
    }
}

const RAVEN_STAND: [&str; 12] = [
    "....XXXX......",
    "...XXXXXX.....",
    "..XXXXXXXX....",
    "..XX.XXXXX....",
    ".XXXXXXXXX....",
    "XXXXXXXXXX....",
    "..XXXXXXXXX...",
    "...XXXXXXX....",
    "....XXXXX.....",
    "....XX.XX.....",
    "...XX...XX....",
    "..XX.....XX...",
];

const RAVEN_STEP: [&str; 12] = [
    ".....XXXX.....",
    "....XXXXXX....",
    "...XXXXXXXX...",
    "...XX.XXXX....",
    "..XXXXXXXX....",
    ".XXXXXXXXX....",
    "...XXXXXXXX...",
    "....XXXXXX....",
    ".....XXXX.....",
    "....XX..XX....",
    "...XX.....X...",
    "....XX........",
];

const RAVEN_BLINK: [&str; 12] = [
    "....XXXX......",
    "...XXXXXX.....",
    "..XXXXXXXX....",
    "..XXXXXXXX....",
    ".XXXXXXXXX....",
    "XXXXXXXXXX....",
    "..XXXXXXXXX...",
    "...XXXXXXX....",
    "....XXXXX.....",
    "....XX.XX.....",
    "...XX...XX....",
    "..XX.....XX...",
];

/// Draws the small raven companion sprite.
pub fn raven(canvas: &mut Canvas, x: i32, y: i32, scale: i32, ctx: FrameContext, color: Color) {
    let frame = match ctx.frame % 18 {
        0..=2 => &RAVEN_STEP,
        11 => &RAVEN_BLINK,
        _ => &RAVEN_STAND,
    };
    draw_sprite(canvas, frame, x, y, scale, color);
}

/// Draws the wake animation raven traveling across the screen.
pub fn raven_wake(canvas: &mut Canvas, ctx: FrameContext, color: Color) {
    let total = ctx.total.max(1);
    let progress = ctx.frame.min(total) as i32;
    let x = -60 + progress * (WIDTH as i32 + 140) / total as i32;
    let hop = if ctx.frame > total.saturating_sub(8) {
        -((ctx.frame - total.saturating_sub(8)) as i32 * 7)
    } else {
        -ctx.pulse(12, 14)
    };
    let ground_y = HEIGHT as i32 - 84;
    canvas.line(0, ground_y, WIDTH as i32, ground_y, Color(46));
    raven(canvas, x, ground_y - 48 + hop, 4, ctx, color);

    for i in 0..6 {
        let fx = x - 16 - i * 28;
        if fx > 0 && fx < WIDTH as i32 {
            canvas.fill_rect(Rect::new(fx, ground_y + 22 + (i % 2) * 3, 7, 3), Color(58));
        }
    }
}

/// Draws a pulsing listening ring.
pub fn listening_rings(canvas: &mut Canvas, cx: i32, cy: i32, ctx: FrameContext) {
    for i in 0..3 {
        let radius = 24 + ((ctx.frame as i32 * 3 + i * 18) % 58);
        let shade = Color(110 + i as u8 * 28);
        canvas.stroke_rect(Rect::new(cx - radius, cy - radius, radius * 2, radius * 2), shade);
    }
}

fn draw_sprite(canvas: &mut Canvas, sprite: &[&str], x: i32, y: i32, scale: i32, color: Color) {
    for (row, line) in sprite.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'X' {
                canvas.fill_rect(
                    Rect::new(
                        x + col as i32 * scale,
                        y + row as i32 * scale,
                        scale,
                        scale,
                    ),
                    color,
                );
            }
        }
    }
}

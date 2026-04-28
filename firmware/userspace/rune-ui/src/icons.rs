//! Small monochrome icons drawn with primitives.

use crate::canvas::{Canvas, Color, Rect};

/// Application icon identifiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Icon {
    /// Speech bubble for Ask Rune.
    Ask,
    /// Reader book.
    Reader,
    /// Music note.
    Music,
    /// Notification bell.
    Notifications,
    /// Capture camera.
    Capture,
    /// Battery icon.
    Battery,
    /// Clock icon.
    Clock,
    /// Back chevron.
    Back,
    /// Forward chevron.
    Chevron,
    /// Send arrow.
    Send,
}

/// Draws an icon in a 20x20-ish box.
pub fn draw(canvas: &mut Canvas, icon: Icon, x: i32, y: i32, color: Color) {
    match icon {
        Icon::Ask => {
            canvas.stroke_circle(x + 10, y + 10, 7, color);
            canvas.line(x + 5, y + 15, x + 3, y + 19, color);
            canvas.line(x + 5, y + 15, x + 8, y + 17, color);
        }
        Icon::Reader => {
            canvas.stroke_round_rect(Rect::new(x + 3, y + 4, 14, 13), 2, color);
            canvas.line(x + 10, y + 4, x + 10, y + 17, color);
            canvas.line(x + 4, y + 5, x + 10, y + 4, color);
            canvas.line(x + 10, y + 4, x + 16, y + 5, color);
        }
        Icon::Music => {
            canvas.line(x + 13, y + 3, x + 13, y + 14, color);
            canvas.line(x + 13, y + 3, x + 18, y + 5, color);
            canvas.fill_circle(x + 8, y + 15, 3, color);
            canvas.line(x + 8, y + 15, x + 13, y + 13, color);
        }
        Icon::Notifications => {
            canvas.line(x + 5, y + 16, x + 15, y + 16, color);
            canvas.stroke_round_rect(Rect::new(x + 6, y + 6, 8, 10), 3, color);
            canvas.line(x + 8, y + 6, x + 10, y + 3, color);
            canvas.line(x + 12, y + 6, x + 10, y + 3, color);
            canvas.pixel(x + 10, y + 18, color);
        }
        Icon::Capture => {
            canvas.stroke_round_rect(Rect::new(x + 2, y + 6, 16, 12), 2, color);
            canvas.line(x + 7, y + 6, x + 8, y + 3, color);
            canvas.line(x + 8, y + 3, x + 12, y + 3, color);
            canvas.line(x + 12, y + 3, x + 13, y + 6, color);
            canvas.stroke_circle(x + 10, y + 12, 3, color);
        }
        Icon::Battery => {
            canvas.stroke_round_rect(Rect::new(x, y + 6, 18, 8), 2, color);
            canvas.fill_round_rect(Rect::new(x + 19, y + 9, 2, 3), 1, color);
        }
        Icon::Clock => {
            canvas.stroke_circle(x + 10, y + 10, 7, color);
            canvas.line(x + 10, y + 6, x + 10, y + 10, color);
            canvas.line(x + 10, y + 10, x + 13, y + 12, color);
        }
        Icon::Back => {
            canvas.line(x + 12, y + 5, x + 7, y + 10, color);
            canvas.line(x + 7, y + 10, x + 12, y + 15, color);
        }
        Icon::Chevron => {
            canvas.line(x + 7, y + 5, x + 12, y + 10, color);
            canvas.line(x + 12, y + 10, x + 7, y + 15, color);
        }
        Icon::Send => {
            canvas.line(x + 4, y + 10, x + 16, y + 10, color);
            canvas.line(x + 12, y + 6, x + 16, y + 10, color);
            canvas.line(x + 16, y + 10, x + 12, y + 14, color);
        }
    }
}

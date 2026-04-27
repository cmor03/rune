//! Small monochrome icons drawn with primitives.

use crate::canvas::{Canvas, Color, Rect};

/// Application icon identifiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Icon {
    /// Voice query.
    Voice,
    /// Book reader.
    Book,
    /// Music playback.
    Music,
    /// Notification mirroring.
    Bell,
    /// Camera capture.
    Camera,
    /// Settings.
    Settings,
    /// Battery indicator.
    Battery,
}

/// Draws an icon in a 24x24-ish box.
pub fn draw(canvas: &mut Canvas, icon: Icon, x: i32, y: i32, color: Color) {
    match icon {
        Icon::Voice => {
            canvas.stroke_rect(Rect::new(x + 8, y + 3, 8, 13), color);
            canvas.line(x + 6, y + 11, x + 6, y + 14, color);
            canvas.line(x + 18, y + 11, x + 18, y + 14, color);
            canvas.line(x + 8, y + 18, x + 16, y + 18, color);
            canvas.line(x + 12, y + 16, x + 12, y + 21, color);
        }
        Icon::Book => {
            canvas.stroke_rect(Rect::new(x + 3, y + 4, 8, 16), color);
            canvas.stroke_rect(Rect::new(x + 12, y + 4, 8, 16), color);
            canvas.line(x + 11, y + 5, x + 11, y + 20, color);
        }
        Icon::Music => {
            canvas.line(x + 14, y + 3, x + 14, y + 16, color);
            canvas.line(x + 14, y + 3, x + 20, y + 5, color);
            canvas.fill_circle(x + 9, y + 17, 4, color);
            canvas.line(x + 9, y + 17, x + 14, y + 15, color);
        }
        Icon::Bell => {
            canvas.line(x + 7, y + 17, x + 17, y + 17, color);
            canvas.stroke_rect(Rect::new(x + 8, y + 7, 8, 10), color);
            canvas.pixel(x + 12, y + 20, color);
        }
        Icon::Camera => {
            canvas.stroke_rect(Rect::new(x + 4, y + 7, 16, 12), color);
            canvas.fill_rect(Rect::new(x + 7, y + 5, 5, 3), color);
            canvas.fill_circle(x + 12, y + 13, 4, color);
            canvas.fill_circle(x + 12, y + 13, 2, Color(239));
        }
        Icon::Settings => {
            canvas.fill_circle(x + 12, y + 12, 7, color);
            canvas.fill_circle(x + 12, y + 12, 3, Color(239));
            canvas.line(x + 12, y + 1, x + 12, y + 5, color);
            canvas.line(x + 12, y + 19, x + 12, y + 23, color);
            canvas.line(x + 1, y + 12, x + 5, y + 12, color);
            canvas.line(x + 19, y + 12, x + 23, y + 12, color);
        }
        Icon::Battery => {
            canvas.stroke_rect(Rect::new(x, y + 6, 18, 10), color);
            canvas.fill_rect(Rect::new(x + 19, y + 9, 2, 4), color);
            canvas.fill_rect(Rect::new(x + 3, y + 9, 11, 4), color);
        }
    }
}

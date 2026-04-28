//! Embedded outline font rendering.

use std::sync::OnceLock;

use ab_glyph::{point, Font, FontArc, GlyphId, PxScale, ScaleFont};

use crate::canvas::{Canvas, Color};

/// Font faces used by the Rune UI mock.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Face {
    /// DM Sans, used for most interface text.
    Sans,
    /// DM Mono Light, used for large clock numerals.
    MonoLight,
    /// DM Mono Regular, used for small numeric metadata.
    Mono,
    /// Lora Italic, used for the Rune wordmark and softer headings.
    SerifItalic,
}

pub(crate) struct TextStyle {
    face: Face,
    size: f32,
    tracking: f32,
    color: Color,
}

impl TextStyle {
    pub(crate) const fn new(face: Face, size: f32, tracking: f32, color: Color) -> Self {
        Self {
            face,
            size,
            tracking,
            color,
        }
    }
}

pub(crate) fn measure(face: Face, text: &str, size: f32, tracking: f32) -> f32 {
    let Some(font) = font(face) else {
        return 0.0;
    };
    let scaled = font.as_scaled(PxScale::from(size));
    let mut cursor = 0.0;
    let mut previous = None;
    let mut drawn = 0;

    for ch in text.chars() {
        if ch == '\n' {
            break;
        }
        let glyph_id = scaled.glyph_id(ch);
        cursor += kerning(&scaled, previous, glyph_id);
        cursor += scaled.h_advance(glyph_id);
        cursor += tracking;
        previous = Some(glyph_id);
        drawn += 1;
    }

    if drawn > 0 {
        cursor - tracking
    } else {
        0.0
    }
}

pub(crate) fn draw(canvas: &mut Canvas, x: i32, y: i32, text: &str, style: TextStyle) {
    let Some(font) = font(style.face) else {
        return;
    };
    let scaled = font.as_scaled(PxScale::from(style.size));
    let line_height = (scaled.ascent() - scaled.descent() + scaled.line_gap()).ceil();
    let origin_x = x as f32;
    let mut baseline = y as f32 + scaled.ascent();
    let mut cursor = origin_x;
    let mut previous = None;

    for ch in text.chars() {
        if ch == '\n' {
            baseline += line_height;
            cursor = origin_x;
            previous = None;
            continue;
        }

        let glyph_id = scaled.glyph_id(ch);
        cursor += kerning(&scaled, previous, glyph_id);
        let glyph =
            glyph_id.with_scale_and_position(PxScale::from(style.size), point(cursor, baseline));

        if let Some(outlined) = font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            outlined.draw(|gx, gy, coverage| {
                canvas.blend_pixel(
                    bounds.min.x as i32 + gx as i32,
                    bounds.min.y as i32 + gy as i32,
                    style.color,
                    coverage,
                );
            });
        }

        cursor += scaled.h_advance(glyph_id) + style.tracking;
        previous = Some(glyph_id);
    }
}

fn kerning<F>(font: &F, previous: Option<GlyphId>, current: GlyphId) -> f32
where
    F: ScaleFont,
{
    previous.map_or(0.0, |prev| font.kern(prev, current))
}

fn font(face: Face) -> Option<&'static FontArc> {
    static SANS: OnceLock<Option<FontArc>> = OnceLock::new();
    static MONO_LIGHT: OnceLock<Option<FontArc>> = OnceLock::new();
    static MONO: OnceLock<Option<FontArc>> = OnceLock::new();
    static SERIF_ITALIC: OnceLock<Option<FontArc>> = OnceLock::new();

    let slot = match face {
        Face::Sans => SANS.get_or_init(|| load(include_bytes!("../assets/fonts/DMSans.ttf"))),
        Face::MonoLight => {
            MONO_LIGHT.get_or_init(|| load(include_bytes!("../assets/fonts/DMMono-Light.ttf")))
        }
        Face::Mono => MONO.get_or_init(|| load(include_bytes!("../assets/fonts/DMMono-Regular.ttf"))),
        Face::SerifItalic => {
            SERIF_ITALIC.get_or_init(|| load(include_bytes!("../assets/fonts/Lora-Italic.ttf")))
        }
    };
    slot.as_ref()
}

fn load(bytes: &'static [u8]) -> Option<FontArc> {
    FontArc::try_from_slice(bytes).ok()
}

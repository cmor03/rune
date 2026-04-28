//! Rune UI grayscale palette and spacing constants.

use crate::canvas::Color;

/// Warm off-white e-ink background from the handoff.
pub const PAPER: Color = Color(238);
/// Pressed or recessed warm paper.
pub const PAPER_DIM: Color = Color(226);
/// Hairline borders and separators.
pub const LINE: Color = Color(217);
/// Secondary text.
pub const MUTED: Color = Color(148);
/// Tertiary text.
pub const FAINT: Color = Color(180);
/// Primary warm ink.
pub const INK: Color = Color(22);
/// Deep warm black.
pub const BLACK: Color = Color(23);
/// Full white.
pub const WHITE: Color = Color(255);
/// Focus fill used for selected controls.
pub const FOCUS: Color = Color(22);
/// Alert or active dark gray.
pub const ACTIVE: Color = Color(64);

/// Outer content margin.
pub const MARGIN: i32 = 12;
/// Standard corner radius approximation, expressed as inset pixels.
pub const RADIUS: i32 = 4;

//! Rune UI grayscale palette and spacing constants.

use crate::canvas::Color;

/// Off-white e-ink background.
pub const PAPER: Color = Color(239);
/// Slightly darker panel background.
pub const PAPER_DIM: Color = Color(226);
/// Hairline borders and separators.
pub const LINE: Color = Color(198);
/// Secondary text.
pub const MUTED: Color = Color(116);
/// Primary ink.
pub const INK: Color = Color(24);
/// Full black.
pub const BLACK: Color = Color(0);
/// Full white.
pub const WHITE: Color = Color(255);
/// Focus fill used for selected controls.
pub const FOCUS: Color = Color(32);
/// Alert or active dark gray.
pub const ACTIVE: Color = Color(64);

/// Outer content margin.
pub const MARGIN: i32 = 12;
/// Standard corner radius approximation, expressed as inset pixels.
pub const RADIUS: i32 = 4;

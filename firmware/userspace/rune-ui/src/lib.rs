//! Portable Rune UI renderer.
//!
//! This crate deliberately renders into a plain 480x280 grayscale framebuffer.
//! The same output can be saved as an image, copied to a Raspberry Pi LCD
//! framebuffer, or converted into the byte layout required by the final e-ink
//! panel.

pub mod animation;
pub mod app;
pub mod canvas;
pub mod display;
pub mod font;
pub mod icons;
pub mod screens;
pub mod theme;

pub use app::{render, Screen};
pub use canvas::{Canvas, Color, Rect, HEIGHT, WIDTH};

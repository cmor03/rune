//! Top-level UI state and render entry point.

use crate::animation::FrameContext;
use crate::canvas::Canvas;
use crate::screens;
use crate::theme;

/// Rune screens available in the first portable mock.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Screen {
    /// Low-power clock screen.
    Sleep,
    /// Companion wake animation.
    Wake,
    /// Main five-function launcher.
    Home,
    /// Voice query flow.
    Voice,
    /// ePub reading view.
    Reader,
    /// Music playback view.
    Music,
    /// Phone notification mirror.
    Notifications,
    /// Photo capture view.
    Camera,
}

impl Screen {
    /// Parses a screen name.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "sleep" => Some(Self::Sleep),
            "wake" => Some(Self::Wake),
            "home" => Some(Self::Home),
            "voice" => Some(Self::Voice),
            "reader" => Some(Self::Reader),
            "music" => Some(Self::Music),
            "notifications" | "notifs" => Some(Self::Notifications),
            "camera" | "capture" => Some(Self::Camera),
            _ => None,
        }
    }

    /// Returns a stable lowercase screen name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Sleep => "sleep",
            Self::Wake => "wake",
            Self::Home => "home",
            Self::Voice => "voice",
            Self::Reader => "reader",
            Self::Music => "music",
            Self::Notifications => "notifications",
            Self::Camera => "camera",
        }
    }
}

/// Renders one frame for `screen`.
pub fn render(screen: Screen, ctx: FrameContext) -> Canvas {
    let mut canvas = Canvas::new(theme::PAPER);
    match screen {
        Screen::Sleep => screens::sleep(&mut canvas, ctx),
        Screen::Wake => screens::wake(&mut canvas, ctx),
        Screen::Home => screens::home(&mut canvas, ctx),
        Screen::Voice => screens::voice(&mut canvas, ctx),
        Screen::Reader => screens::reader(&mut canvas, ctx),
        Screen::Music => screens::music(&mut canvas, ctx),
        Screen::Notifications => screens::notifications(&mut canvas, ctx),
        Screen::Camera => screens::camera(&mut canvas, ctx),
    }
    canvas.surface_texture(2);
    canvas
}

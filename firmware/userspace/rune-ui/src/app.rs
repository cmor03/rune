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

/// Simulated wheel/button input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    /// Wheel up.
    Up,
    /// Wheel down.
    Down,
    /// Short press.
    Press,
    /// Press and hold.
    PressHold,
}

impl Command {
    /// Parses a command string.
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "UP" => Some(Self::Up),
            "DOWN" => Some(Self::Down),
            "PRESS" => Some(Self::Press),
            "PHOLD" | "PRESS_HOLD" | "PRESSHOLD" => Some(Self::PressHold),
            _ => None,
        }
    }
}

/// Stateful UI model used by the command loop.
#[derive(Clone, Debug)]
pub struct UiState {
    /// Current screen.
    pub screen: Screen,
    /// Home launcher selected tile.
    pub selected: usize,
}

impl UiState {
    /// Creates the initial UI state.
    pub const fn new() -> Self {
        Self {
            screen: Screen::Home,
            selected: 0,
        }
    }

    /// Applies one command.
    pub fn apply(&mut self, command: Command) {
        match command {
            Command::Up => {
                if self.screen == Screen::Home {
                    self.selected = self.selected.saturating_sub(1);
                }
            }
            Command::Down => {
                if self.screen == Screen::Home {
                    self.selected = (self.selected + 1).min(4);
                }
            }
            Command::Press => {
                self.screen = if self.screen == Screen::Home {
                    HOME_SCREENS[self.selected]
                } else {
                    Screen::Home
                };
            }
            Command::PressHold => {
                self.screen = Screen::Voice;
            }
        }
    }
}

const HOME_SCREENS: [Screen; 5] = [
    Screen::Voice,
    Screen::Reader,
    Screen::Music,
    Screen::Notifications,
    Screen::Camera,
];

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
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

/// Renders one frame from state.
pub fn render_state(state: &UiState, ctx: FrameContext) -> Canvas {
    render(state.screen, ctx.with_selected(state.selected))
}

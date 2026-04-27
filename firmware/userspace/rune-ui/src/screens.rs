//! Screen renderers for the portable Rune UI mock.

use crate::animation::{self, FrameContext};
use crate::canvas::{Canvas, Color, Rect};
use crate::icons::{self, Icon};
use crate::theme;

/// Renders the sleep screen.
pub fn sleep(canvas: &mut Canvas, _ctx: FrameContext) {
    canvas.clear(theme::BLACK);
    canvas.text(232, 24, "RUNE", 2, Color(70));
    canvas.text_center(160, 150, "09:41", 7, theme::PAPER);
    canvas.text_center(160, 214, "TUE MAY 13", 2, Color(110));
    canvas.text_center(160, 420, "PRESS TO WAKE", 1, Color(78));
}

/// Renders the wake animation.
pub fn wake(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(theme::BLACK);
    canvas.text(232, 24, "RUNE", 2, Color(70));
    animation::raven_wake(canvas, ctx, Color(214));
    canvas.text_center(160, 352, "GOOD MORNING", 2, Color(120));
}

/// Renders the main launcher.
pub fn home(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(theme::PAPER);
    header(canvas, "GOOD MORNING", "09:41", ctx.battery_pct);
    canvas.line(14, 74, 306, 74, theme::LINE);

    canvas.panel(Rect::new(14, 92, 292, 62), theme::PAPER, theme::LINE);
    icons::draw(canvas, Icon::Voice, 28, 112, theme::INK);
    canvas.text(64, 106, "VOICE READY", 2, theme::INK);
    canvas.text(64, 130, "HOLD TO ASK", 1, theme::MUTED);
    canvas.text(282, 116, ">", 3, theme::MUTED);

    let tiles = [
        (Icon::Book, "READER", "EPUB"),
        (Icon::Music, "MUSIC", "LOCAL"),
        (Icon::Bell, "NOTIFS", "PHONE"),
        (Icon::Camera, "CAMERA", "PHOTO"),
        (Icon::Settings, "SETUP", "DEVICE"),
    ];

    for (i, (icon, title, sub)) in tiles.iter().enumerate() {
        let x = 14;
        let y = 174 + i as i32 * 58;
        let selected = i == ctx.selected;
        tile(canvas, x, y, *icon, title, sub, selected);
    }
}

/// Renders voice query states.
pub fn voice(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(theme::BLACK);
    topbar(canvas, "VOICE", true);
    let pulse = ctx.pulse(24, 8);
    animation::listening_rings(canvas, 160, 190, ctx);
    icons::draw(canvas, Icon::Voice, 148, 168 - pulse / 4, theme::PAPER);
    canvas.text_center(160, 262, "LISTENING", 3, theme::PAPER);
    canvas.text_center(160, 304, "RELEASE TO SEND", 1, Color(120));
}

/// Renders ePub reading.
pub fn reader(canvas: &mut Canvas, _ctx: FrameContext) {
    canvas.clear(theme::PAPER);
    topbar(canvas, "READER", false);
    canvas.text(18, 58, "THE LEFT HAND", 2, theme::INK);
    canvas.line(18, 84, 302, 84, theme::LINE);
    let lines = [
        "THE DEVICE WAS",
        "QUIET BY DESIGN.",
        "IT HELD ONE PAGE",
        "AT A TIME AND",
        "ASKED FOR NOTHING",
        "MORE THAN THE NEXT",
        "DELIBERATE PRESS.",
        "",
        "OUTSIDE THE PHONE",
        "KEPT ITS BRIGHT",
        "LITTLE STORM.",
    ];
    for (i, line) in lines.iter().enumerate() {
        canvas.text(18, 108 + i as i32 * 24, line, 2, theme::INK);
    }
    canvas.line(18, 438, 302, 438, theme::LINE);
    canvas.text(18, 454, "PAGE 12", 1, theme::MUTED);
    canvas.text(260, 454, "34%", 1, theme::MUTED);
}

/// Renders music playback.
pub fn music(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(theme::PAPER);
    topbar(canvas, "MUSIC", false);
    canvas.panel(Rect::new(72, 76, 176, 176), theme::PAPER_DIM, theme::LINE);
    icons::draw(canvas, Icon::Music, 148, 150, theme::INK);
    canvas.text_center(160, 282, "QUIET ROOM", 3, theme::INK);
    canvas.text_center(160, 322, "LOCAL FILE", 1, theme::MUTED);
    let progress = 38 + ((ctx.frame as i32 * 5) % 244);
    canvas.line(38, 370, 282, 370, theme::LINE);
    canvas.fill_rect(Rect::new(38, 368, progress - 38, 5), theme::INK);
    canvas.text(38, 392, "01:12", 1, theme::MUTED);
    canvas.text(248, 392, "03:48", 1, theme::MUTED);
}

/// Renders notification mirroring.
pub fn notifications(canvas: &mut Canvas, _ctx: FrameContext) {
    canvas.clear(theme::PAPER);
    topbar(canvas, "NOTIFICATIONS", false);
    notif(canvas, 18, 72, "MESSAGES", "CALL AFTER LUNCH?");
    notif(canvas, 18, 146, "CALENDAR", "DESIGN SYNC IN 19");
    notif(canvas, 18, 220, "PHONE", "LOW PRIORITY ONLY");
    canvas.text_center(160, 430, "PRESS TO DISMISS", 1, theme::MUTED);
}

/// Renders camera capture.
pub fn camera(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(theme::BLACK);
    topbar(canvas, "CAMERA", true);
    canvas.stroke_rect(Rect::new(30, 76, 260, 300), Color(120));
    canvas.line(30, 76, 70, 76, theme::PAPER);
    canvas.line(30, 76, 30, 116, theme::PAPER);
    canvas.line(290, 76, 250, 76, theme::PAPER);
    canvas.line(290, 76, 290, 116, theme::PAPER);
    canvas.line(30, 376, 70, 376, theme::PAPER);
    canvas.line(30, 376, 30, 336, theme::PAPER);
    canvas.line(290, 376, 250, 376, theme::PAPER);
    canvas.line(290, 376, 290, 336, theme::PAPER);
    let scan_y = 92 + (ctx.frame as i32 * 5) % 260;
    canvas.line(48, scan_y, 272, scan_y, Color(170));
    canvas.text_center(160, 414, "PHOTO   TEXT   SCAN", 1, theme::PAPER);
}

fn header(canvas: &mut Canvas, greeting: &str, time: &str, battery: u8) {
    canvas.text(14, 16, greeting, 1, theme::MUTED);
    canvas.text(14, 34, time, 5, theme::INK);
    icons::draw(canvas, Icon::Battery, 242, 38, theme::INK);
    canvas.text(274, 44, &format!("{battery}%"), 1, theme::MUTED);
}

fn topbar(canvas: &mut Canvas, title: &str, dark: bool) {
    let fg = if dark { theme::PAPER } else { theme::INK };
    let line = if dark { Color(64) } else { theme::LINE };
    canvas.text(14, 18, "<", 3, fg);
    canvas.text(46, 22, title, 2, fg);
    canvas.text(252, 24, "09:41", 1, if dark { Color(130) } else { theme::MUTED });
    canvas.line(14, 50, 306, 50, line);
}

fn tile(canvas: &mut Canvas, x: i32, y: i32, icon: Icon, title: &str, sub: &str, selected: bool) {
    let fill = if selected { theme::FOCUS } else { theme::PAPER };
    let fg = if selected { theme::PAPER } else { theme::INK };
    let muted = if selected { Color(190) } else { theme::MUTED };
    canvas.panel(Rect::new(x, y, 292, 48), fill, theme::LINE);
    icons::draw(canvas, icon, x + 10, y + 11, fg);
    canvas.text(x + 42, y + 10, title, 2, fg);
    canvas.text(x + 42, y + 31, sub, 1, muted);
}

fn notif(canvas: &mut Canvas, x: i32, y: i32, label: &str, body: &str) {
    canvas.panel(Rect::new(x, y, 284, 54), theme::PAPER, theme::LINE);
    icons::draw(canvas, Icon::Bell, x + 12, y + 10, theme::INK);
    canvas.text(x + 48, y + 9, label, 1, theme::MUTED);
    canvas.text(x + 48, y + 24, body, 2, theme::INK);
}

//! Screen renderers for the portable Rune UI mock.

use crate::animation::{self, FrameContext};
use crate::canvas::{Canvas, Color, Rect};
use crate::icons::{self, Icon};
use crate::theme;

/// Renders the sleep screen.
pub fn sleep(canvas: &mut Canvas, _ctx: FrameContext) {
    canvas.clear(theme::BLACK);
    canvas.text(348, 24, "RUNE", 2, Color(70));
    canvas.text_center(240, 90, "09:41", 8, theme::PAPER);
    canvas.text_center(240, 158, "TUE MAY 13", 2, Color(110));
    canvas.text_center(240, 238, "PRESS TO WAKE", 1, Color(78));
}

/// Renders the wake animation.
pub fn wake(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(theme::BLACK);
    canvas.text(348, 24, "RUNE", 2, Color(70));
    animation::raven_wake(canvas, ctx, Color(214));
    canvas.text_center(240, 232, "GOOD MORNING", 2, Color(120));
}

/// Renders the main launcher.
pub fn home(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(theme::PAPER);
    header(canvas, "GOOD MORNING", "09:41", ctx.battery_pct);
    canvas.line(18, 72, 462, 72, theme::LINE);

    canvas.panel(Rect::new(18, 86, 444, 54), theme::PAPER, theme::LINE);
    icons::draw(canvas, Icon::Voice, 32, 101, theme::INK);
    canvas.text(66, 98, "VOICE READY", 2, theme::INK);
    canvas.text(66, 120, "HOLD ACTION TO ASK RUNE", 1, theme::MUTED);
    canvas.text(438, 106, ">", 3, theme::MUTED);

    let tiles = [
        (Icon::Voice, "VOICE", "QUERY"),
        (Icon::Book, "READER", "EPUB"),
        (Icon::Music, "MUSIC", "LOCAL"),
        (Icon::Bell, "NOTIFS", "PHONE"),
        (Icon::Camera, "CAMERA", "PHOTO"),
        (Icon::Settings, "SYSTEM", "SETUP"),
    ];

    for (i, (icon, title, sub)) in tiles.iter().enumerate() {
        let col = (i % 3) as i32;
        let row = (i / 3) as i32;
        let x = 18 + col * 150;
        let y = 154 + row * 58;
        let selected = i == (ctx.frame as usize / 8) % tiles.len();
        tile(canvas, x, y, *icon, title, sub, selected);
    }
}

/// Renders voice query states.
pub fn voice(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(theme::BLACK);
    topbar(canvas, "VOICE", true);
    let pulse = ctx.pulse(24, 8);
    animation::listening_rings(canvas, 240, 130, ctx);
    icons::draw(canvas, Icon::Voice, 228, 108 - pulse / 4, theme::PAPER);
    canvas.text_center(240, 180, "LISTENING", 3, theme::PAPER);
    canvas.text_center(240, 214, "RELEASE TO SEND", 1, Color(120));
}

/// Renders ePub reading.
pub fn reader(canvas: &mut Canvas, _ctx: FrameContext) {
    canvas.clear(theme::PAPER);
    topbar(canvas, "READER", false);
    canvas.text(28, 52, "THE LEFT HAND", 2, theme::INK);
    canvas.line(28, 76, 452, 76, theme::LINE);
    let lines = [
        "THE DEVICE WAS QUIET BY",
        "DESIGN. IT HELD ONE PAGE",
        "AT A TIME AND ASKED FOR",
        "NOTHING MORE THAN THE",
        "NEXT DELIBERATE PRESS.",
        "",
        "OUTSIDE, THE PHONE KEPT",
        "ITS BRIGHT LITTLE STORM.",
    ];
    for (i, line) in lines.iter().enumerate() {
        canvas.text(28, 94 + i as i32 * 20, line, 2, theme::INK);
    }
    canvas.line(28, 246, 452, 246, theme::LINE);
    canvas.text(28, 258, "PAGE 12", 1, theme::MUTED);
    canvas.text(398, 258, "34%", 1, theme::MUTED);
}

/// Renders music playback.
pub fn music(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(theme::PAPER);
    topbar(canvas, "MUSIC", false);
    canvas.panel(Rect::new(38, 62, 132, 132), theme::PAPER_DIM, theme::LINE);
    icons::draw(canvas, Icon::Music, 92, 112, theme::INK);
    canvas.text(202, 76, "LOCAL FILE", 1, theme::MUTED);
    canvas.text(202, 98, "QUIET ROOM", 3, theme::INK);
    canvas.text(202, 132, "NO RECOMMENDATIONS", 1, theme::MUTED);
    let progress = 202 + ((ctx.frame as i32 * 5) % 208);
    canvas.line(202, 184, 430, 184, theme::LINE);
    canvas.fill_rect(Rect::new(202, 182, progress - 202, 5), theme::INK);
    canvas.text(202, 202, "01:12", 1, theme::MUTED);
    canvas.text(396, 202, "03:48", 1, theme::MUTED);
}

/// Renders notification mirroring.
pub fn notifications(canvas: &mut Canvas, _ctx: FrameContext) {
    canvas.clear(theme::PAPER);
    topbar(canvas, "NOTIFICATIONS", false);
    notif(canvas, 30, 62, "MESSAGES", "CAN YOU CALL AFTER LUNCH?");
    notif(canvas, 30, 122, "CALENDAR", "DESIGN SYNC IN 19 MIN");
    notif(canvas, 30, 182, "PHONE", "LOW PRIORITY MIRROR ONLY");
    canvas.text_center(240, 252, "DISMISS ON PHONE OR RUNE", 1, theme::MUTED);
}

/// Renders camera capture.
pub fn camera(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(theme::BLACK);
    topbar(canvas, "CAMERA", true);
    canvas.stroke_rect(Rect::new(42, 58, 396, 164), Color(120));
    canvas.line(42, 58, 82, 58, theme::PAPER);
    canvas.line(42, 58, 42, 98, theme::PAPER);
    canvas.line(438, 58, 398, 58, theme::PAPER);
    canvas.line(438, 58, 438, 98, theme::PAPER);
    canvas.line(42, 222, 82, 222, theme::PAPER);
    canvas.line(42, 222, 42, 182, theme::PAPER);
    canvas.line(438, 222, 398, 222, theme::PAPER);
    canvas.line(438, 222, 438, 182, theme::PAPER);
    let scan_y = 70 + (ctx.frame as i32 * 5) % 138;
    canvas.line(60, scan_y, 420, scan_y, Color(170));
    canvas.text_center(240, 240, "PHOTO   TEXT   SCAN", 2, theme::PAPER);
}

fn header(canvas: &mut Canvas, greeting: &str, time: &str, battery: u8) {
    canvas.text(18, 16, greeting, 1, theme::MUTED);
    canvas.text(18, 32, time, 5, theme::INK);
    icons::draw(canvas, Icon::Battery, 380, 34, theme::INK);
    canvas.text(414, 40, &format!("{battery}%"), 1, theme::MUTED);
}

fn topbar(canvas: &mut Canvas, title: &str, dark: bool) {
    let fg = if dark { theme::PAPER } else { theme::INK };
    let line = if dark { Color(64) } else { theme::LINE };
    canvas.text(18, 18, "<", 3, fg);
    canvas.text(54, 22, title, 2, fg);
    canvas.text(400, 24, "09:41", 1, if dark { Color(130) } else { theme::MUTED });
    canvas.line(18, 46, 462, 46, line);
}

fn tile(canvas: &mut Canvas, x: i32, y: i32, icon: Icon, title: &str, sub: &str, selected: bool) {
    let fill = if selected { theme::FOCUS } else { theme::PAPER };
    let fg = if selected { theme::PAPER } else { theme::INK };
    let muted = if selected { Color(190) } else { theme::MUTED };
    canvas.panel(Rect::new(x, y, 132, 48), fill, theme::LINE);
    icons::draw(canvas, icon, x + 10, y + 11, fg);
    canvas.text(x + 42, y + 10, title, 2, fg);
    canvas.text(x + 42, y + 31, sub, 1, muted);
}

fn notif(canvas: &mut Canvas, x: i32, y: i32, label: &str, body: &str) {
    canvas.panel(Rect::new(x, y, 420, 44), theme::PAPER, theme::LINE);
    icons::draw(canvas, Icon::Bell, x + 12, y + 10, theme::INK);
    canvas.text(x + 48, y + 9, label, 1, theme::MUTED);
    canvas.text(x + 48, y + 24, body, 2, theme::INK);
}

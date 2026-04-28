//! Screen renderers for the portable Rune UI mock.

use crate::animation::{self, FrameContext};
use crate::canvas::{Canvas, Color, Rect, WIDTH};
use crate::font::{self, Face};
use crate::icons::{self, Icon};
use crate::theme;

const INSET: i32 = 17;
const SCREEN_TIME: &str = "09:41";

struct AppTile {
    icon: Icon,
    label: &'static str,
    badge: Option<&'static str>,
}

const HOME_APPS: [AppTile; 5] = [
    AppTile {
        icon: Icon::Ask,
        label: "Ask",
        badge: None,
    },
    AppTile {
        icon: Icon::Reader,
        label: "Reader",
        badge: None,
    },
    AppTile {
        icon: Icon::Music,
        label: "Music",
        badge: None,
    },
    AppTile {
        icon: Icon::Notifications,
        label: "Notifs",
        badge: Some("2"),
    },
    AppTile {
        icon: Icon::Capture,
        label: "Capture",
        badge: None,
    },
];

/// Renders the sleep screen.
pub fn sleep(canvas: &mut Canvas, _ctx: FrameContext) {
    canvas.clear(theme::BLACK);
    wordmark(canvas, 248, 18, Color(62));
    canvas.text_center(160, 185, SCREEN_TIME, Face::MonoLight, 72.0, theme::PAPER);
    canvas.text_center_tracked(160, 262, "Mon, May 12", Face::Sans, 11.5, 1.0, Color(106));
    canvas.text_center_tracked(160, 444, "PRESS TO WAKE", Face::Sans, 9.0, 2.1, Color(62));
}

/// Renders the wake animation.
pub fn wake(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(theme::BLACK);
    wordmark(canvas, 248, 18, Color(62));
    animation::raven_wake(canvas, ctx, Color(216));
}

/// Renders the main launcher.
pub fn home(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(theme::PAPER);
    canvas.text(19, 17, "Good morning", Face::SerifItalic, 16.0, Color(102));
    canvas.text(19, 39, SCREEN_TIME, Face::MonoLight, 39.0, theme::INK);
    canvas.text_tracked(20, 79, "Mon, May 12", Face::Sans, 10.0, 0.6, theme::MUTED);
    battery(canvas, 224, 53, ctx.battery_pct);
    canvas.line(INSET, 101, WIDTH as i32 - INSET, 101, theme::LINE);

    context_strip(canvas);
    canvas.text_tracked(19, 198, "APPS", Face::Sans, 9.0, 1.6, Color(160));

    let tile_w = 89;
    let tile_h = 104;
    let gap = 10;
    let grid_x = INSET;
    let grid_y = 221;
    for (idx, app) in HOME_APPS.iter().enumerate() {
        let col = idx as i32 % 3;
        let row = idx as i32 / 3;
        let x = grid_x + col * (tile_w + gap);
        let y = grid_y + row * (tile_h + gap);
        app_tile(canvas, x, y, tile_w, tile_h, app, idx == ctx.selected);
    }

    wordmark(canvas, 139, 444, theme::FAINT);
}

/// Renders voice query states.
pub fn voice(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(Color(18));
    wordmark(canvas, 248, 18, Color(58));
    listening_rings(canvas, 160, 206, ctx);
    animation::raven(canvas, 118, 168 - ctx.pulse(24, 4), 5, ctx, Color(216));
    canvas.text_center_tracked(160, 286, "LISTENING", Face::Mono, 14.0, 2.2, Color(112));
    canvas.text_center(160, 315, "Release to send", Face::SerifItalic, 11.0, Color(58));
}

/// Renders ePub reading.
pub fn reader(canvas: &mut Canvas, _ctx: FrameContext) {
    canvas.clear(theme::PAPER);
    reading_topbar(canvas, "The Left Hand", "34%");
    canvas.text_tracked(18, 73, "CHAPTER 4 - THE HONOR CHASM", Face::Sans, 9.0, 1.2, theme::MUTED);

    let sample = [
        "In the end, what separated a city from the wilderness was not walls or cobblestones or the smell of bread from bakeries.",
        "It was the belief that here, people chose to be near one another.",
        "The device was quiet by design. It held one page at a time and asked for nothing more than the next deliberate press.",
    ];
    let mut y = 96;
    for paragraph in sample {
        y = draw_wrapped(
            canvas,
            paragraph,
            Rect::new(18, y, 284, 0),
            Face::SerifItalic,
            16.0,
            5,
            theme::INK,
        );
        y += 13;
    }

    canvas.line(INSET, 437, WIDTH as i32 - INSET, 437, theme::LINE);
    small_button(canvas, 18, 449, "-");
    small_button(canvas, 46, 449, "+");
    canvas.text_center(160, 452, "p. 312 of 917", Face::Mono, 9.0, theme::FAINT);
    canvas.text_right(302, 452, SCREEN_TIME, Face::Mono, 9.0, theme::MUTED);
}

/// Renders music playback.
pub fn music(canvas: &mut Canvas, ctx: FrameContext) {
    canvas.clear(theme::PAPER);
    topbar(canvas, "Music", false);
    canvas.round_panel(Rect::new(74, 85, 172, 172), 9, theme::PAPER_DIM, theme::LINE);
    icons::draw(canvas, Icon::Music, 150, 150, theme::INK);
    canvas.text_center(160, 285, "Quiet Room", Face::Sans, 18.0, theme::INK);
    canvas.text_center_tracked(160, 314, "LOCAL FILE", Face::Sans, 9.0, 1.5, theme::MUTED);

    let progress = 38 + ((ctx.frame as i32 * 5) % 244);
    canvas.fill_round_rect(Rect::new(38, 366, 244, 3), 1, theme::LINE);
    canvas.fill_round_rect(Rect::new(38, 366, progress - 38, 3), 1, theme::INK);
    canvas.text(38, 386, "01:12", Face::Mono, 10.0, theme::MUTED);
    canvas.text_right(282, 386, "03:48", Face::Mono, 10.0, theme::MUTED);
}

/// Renders notification mirroring.
pub fn notifications(canvas: &mut Canvas, _ctx: FrameContext) {
    canvas.clear(theme::PAPER);
    topbar(canvas, "Notifications", false);
    notification_row(canvas, 62, "Messages", "Call after lunch?", "10:21");
    notification_row(canvas, 122, "Calendar", "Design sync in 19 min", "09:58");
    notification_row(canvas, 182, "Phone", "Low priority only", "Yesterday");
    canvas.round_panel(Rect::new(17, 264, 286, 42), 6, theme::PAPER, Color(196));
    icons::draw(canvas, Icon::Ask, 29, 274, theme::FAINT);
    canvas.text(58, 275, "Ask Rune about this", Face::Sans, 11.0, theme::MUTED);
    canvas.text_center_tracked(160, 434, "PRESS TO DISMISS", Face::Sans, 9.0, 1.6, theme::MUTED);
}

/// Renders camera capture.
pub fn camera(canvas: &mut Canvas, _ctx: FrameContext) {
    canvas.clear(theme::PAPER);
    topbar(canvas, "Capture", false);
    let vf = Rect::new(0, 53, WIDTH as i32, 333);
    canvas.fill_rect(vf, Color(27));
    camera_corner(canvas, 16, 68, true, true);
    camera_corner(canvas, 287, 68, false, true);
    camera_corner(canvas, 16, 350, true, false);
    camera_corner(canvas, 287, 350, false, false);
    canvas.line(155, 215, 165, 215, Color(88));
    canvas.line(160, 210, 160, 220, Color(88));
    canvas.text_center_tracked(105, 356, "SCAN", Face::Sans, 9.0, 1.2, theme::PAPER);
    canvas.text_center_tracked(160, 356, "PHOTO", Face::Sans, 9.0, 1.2, Color(160));
    canvas.text_center_tracked(218, 356, "TEXT", Face::Sans, 9.0, 1.2, Color(160));

    canvas.line(INSET, 386, WIDTH as i32 - INSET, 386, theme::LINE);
    canvas.text_tracked(18, 416, "ALIGN WITHIN FRAME", Face::Sans, 9.0, 0.9, theme::MUTED);
    canvas.stroke_circle(248, 424, 17, theme::INK);
    canvas.fill_circle(248, 424, 11, theme::INK);
    wordmark(canvas, 278, 417, theme::FAINT);
}

fn context_strip(canvas: &mut Canvas) {
    canvas.round_panel(Rect::new(INSET, 113, 286, 68), 7, theme::PAPER, theme::LINE);
    icons::draw(canvas, Icon::Clock, 29, 128, theme::MUTED);
    canvas.text_tracked(61, 125, "NEXT UP", Face::Sans, 9.0, 1.6, Color(160));
    canvas.text(61, 142, "Ask Rune", Face::Sans, 14.0, theme::INK);
    canvas.text(61, 162, "Hold side button to speak", Face::Sans, 10.0, theme::MUTED);
    icons::draw(canvas, Icon::Chevron, 282, 137, theme::FAINT);
}

fn app_tile(canvas: &mut Canvas, x: i32, y: i32, w: i32, h: i32, app: &AppTile, selected: bool) {
    let fill = if selected { theme::FOCUS } else { theme::PAPER };
    let stroke = if selected { theme::FOCUS } else { theme::LINE };
    let fg = if selected { theme::PAPER } else { theme::INK };
    canvas.round_panel(Rect::new(x, y, w, h), 8, fill, stroke);
    icons::draw(canvas, app.icon, x + w / 2 - 10, y + 25, fg);
    canvas.text_center(x + w / 2, y + 61, app.label, Face::Sans, 12.0, fg);

    if let Some(badge) = app.badge {
        let badge_fill = if selected { theme::PAPER } else { theme::INK };
        let badge_fg = if selected { theme::INK } else { theme::PAPER };
        canvas.fill_circle(x + w - 15, y + 14, 8, badge_fill);
        canvas.text_center(x + w - 15, y + 9, badge, Face::Sans, 10.0, badge_fg);
    }
}

fn topbar(canvas: &mut Canvas, title: &str, dark: bool) {
    let fg = if dark { theme::PAPER } else { theme::INK };
    let muted = if dark { Color(96) } else { theme::MUTED };
    let line = if dark { Color(46) } else { theme::LINE };
    icons::draw(canvas, Icon::Back, 15, 17, fg);
    canvas.text(46, 16, title, Face::Sans, 14.0, fg);
    canvas.text_right(303, 18, SCREEN_TIME, Face::Mono, 10.0, muted);
    canvas.line(INSET, 52, WIDTH as i32 - INSET, 52, line);
}

fn reading_topbar(canvas: &mut Canvas, title: &str, progress: &str) {
    icons::draw(canvas, Icon::Back, 15, 17, theme::INK);
    canvas.text(45, 17, title, Face::SerifItalic, 12.0, theme::MUTED);
    canvas.text_right(303, 18, progress, Face::Mono, 10.0, theme::FAINT);
    canvas.line(INSET, 52, WIDTH as i32 - INSET, 52, theme::LINE);
}

fn notification_row(canvas: &mut Canvas, y: i32, title: &str, preview: &str, time: &str) {
    canvas.line(INSET, y + 51, WIDTH as i32 - INSET, y + 51, Color(226));
    canvas.fill_circle(22, y + 15, 3, Color(196));
    canvas.text(37, y + 7, title, Face::Sans, 12.0, theme::INK);
    canvas.text(37, y + 25, preview, Face::Sans, 10.0, theme::MUTED);
    canvas.text_right(303, y + 9, time, Face::Mono, 9.0, theme::FAINT);
}

fn small_button(canvas: &mut Canvas, x: i32, y: i32, label: &str) {
    canvas.round_panel(Rect::new(x, y, 23, 23), 4, theme::PAPER, theme::LINE);
    canvas.text_center(x + 11, y + 1, label, Face::Sans, 18.0, Color(102));
}

fn battery(canvas: &mut Canvas, x: i32, y: i32, pct: u8) {
    canvas.stroke_round_rect(Rect::new(x, y, 22, 9), 2, theme::MUTED);
    canvas.fill_round_rect(Rect::new(x + 24, y + 3, 3, 4), 1, theme::MUTED);
    let fill_w = (18 * pct.min(100) as i32) / 100;
    canvas.fill_round_rect(Rect::new(x + 2, y + 2, fill_w.max(1), 5), 1, theme::INK);
    canvas.text(x + 35, y - 2, &format!("{pct}%"), Face::Mono, 10.0, theme::MUTED);
}

fn wordmark(canvas: &mut Canvas, x: i32, y: i32, color: Color) {
    canvas.text_tracked(x, y, "rune", Face::SerifItalic, 12.0, 0.7, color);
}

fn listening_rings(canvas: &mut Canvas, cx: i32, cy: i32, ctx: FrameContext) {
    for i in 0..3 {
        let radius = 28 + ((ctx.frame as i32 * 3 + i * 21) % 68);
        let shade = Color(48 + i as u8 * 22);
        canvas.stroke_circle(cx, cy, radius, shade);
    }
}

fn camera_corner(canvas: &mut Canvas, x: i32, y: i32, left: bool, top: bool) {
    let color = Color(142);
    let h_end = if left { x + 20 } else { x - 20 };
    let v_end = if top { y + 20 } else { y - 20 };
    canvas.line(x, y, h_end, y, color);
    canvas.line(x, y, x, v_end, color);
}

fn draw_wrapped(
    canvas: &mut Canvas,
    text: &str,
    rect: Rect,
    face: Face,
    size: f32,
    leading: i32,
    color: Color,
) -> i32 {
    let mut cursor_y = rect.y;
    let mut line = String::new();
    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_string()
        } else {
            format!("{line} {word}")
        };
        if line.is_empty() || font::measure(face, &candidate, size, 0.0) <= rect.w as f32 {
            line = candidate;
            continue;
        }
        canvas.text(rect.x, cursor_y, &line, face, size, color);
        cursor_y += size.ceil() as i32 + leading;
        line.clear();
        line.push_str(word);
    }
    if !line.is_empty() {
        canvas.text(rect.x, cursor_y, &line, face, size, color);
        cursor_y += size.ceil() as i32 + leading;
    }
    cursor_y
}
